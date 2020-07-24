/// Job control utilities, including representations of jobs in-memory, a job
/// manager structure, and other such niceties.
/* Notes:
  Most functions currently return Results if they need to signal errors, but
  this isn't really necessary, since every function here can only error in one
  way. This should be refactored.

  ## TODO

  The API for jobslist is not particularly pleasant to work with. You should
  refactor it so that it's a lot easier to use.

  Can this be written in a way that guarantees no race conditions? E.g. using
  an atomic variable to signal the presence/absence of a job, so that no job
  can ever be found in an inconsistent state.
*/
use crate::util::{MAX_JOBID, MAX_NUM_JOBS};
use nix::unistd::Pid;
type Jid = i32;
/// The runstate of a job in the system
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum JobState {
    FG,   // Job is the foreground job
    BG,   // Job is the background job
    Stop, // Job is stopped (from receiving SIGSTOP or SIGTSTP)
}

// The errors that can arise when attempting to manipulate the joblist
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum JobListError {
    TooManyJobs,
    InconsistentState,
    NoSuchPid(Pid), // The latter two options are generally unused in favor of Option
    NoSuchJid(Jid),
}
/// A representation of a single job being controlled by the shell
pub struct JobStruct {
    pid: Pid,
    jid: Jid,
    state: JobState,
    cmdline: String,
}

// Simple getter/setter functions to ensure uniformity
impl JobStruct {
    pub fn jid(&self) -> Jid {
        return self.jid;
    }
    pub fn pid(&self) -> Pid {
        return self.pid;
    }
    pub fn state(&self) -> JobState {
        return self.state;
    }
    pub fn cmdline(&self) -> &str {
        return &self.cmdline;
    }
    pub fn set_state(&mut self, state: JobState) {
        self.state = state;
    }
}

/// A controller for all the active jobs in a given system. Only one of these
/// should exist per shell.
pub struct JobList {
    jobvec: [Option<JobStruct>; MAX_NUM_JOBS],
    nextjid: Jid,
}

impl JobList {
    // Corresponds to initjobs in jobs.c
    pub fn new() -> Self {
        Self {
            jobvec: Default::default(),
            nextjid: 1,
        }
    }
    /// Return the largest JobID that is currently allocated.
    pub fn maxjid(&self) -> Jid {
        self.jobvec
            .iter()
            .filter(|x| x.is_some())
            .map(|x| x.as_ref().unwrap().jid)
            .max()
            .unwrap_or(0)
    }
    /// Adds a job with the given parameters to the joblist. Returns an Err if
    /// there is not enough space for the job.
    pub fn addjob(
        &mut self,
        pid: Pid,
        state: JobState,
        cmdline: &str,
    ) -> Result<Jid, JobListError> {
        let jid = self.nextjid;
        // Ignore validity checks because Pid has checked constructor
        for job_slot in self.jobvec.iter_mut().filter(|x| x.is_none()) {
            *job_slot = Some(JobStruct {
                pid,
                state,
                jid,
                cmdline: cmdline.to_string(),
            });
            // After assigning jobid, increment it, wrapping if needed. Wrapping
            // operates in [1,MAXJOBS] instead of [0, MAXJOBS) so we do custom logic :(
            self.nextjid += 1;
            if self.nextjid > MAX_JOBID {
                self.nextjid = 1;
            }
            let j = (&*job_slot).as_ref().unwrap();
            return Ok(j.jid);
        }
        // No None slots found in the jobvec
        return Err(JobListError::TooManyJobs);
    }

    /// Deletes a job with the requested PID. Returns an Err if there is no such job.
    pub fn deletejob(&mut self, pid: Pid) -> Result<(), JobListError> {
        for job_slot in self.jobvec.iter_mut().filter(|x| x.is_some()) {
            if job_slot.as_mut().unwrap().pid == pid {
                job_slot.take(); // Yoink!
                self.nextjid = self.maxjid() + 1;
                return Ok(());
            }
        }
        return Err(JobListError::NoSuchPid(pid));
    }

    pub fn fgpid(&self) -> Option<Pid> {
        for job in self.jobvec.iter().flat_map(|x| x.iter()) {
            if job.state == JobState::FG {
                return Some(job.pid);
            }
        }
        None
    }

    /// Get a job by its PID
    pub fn getjob_pid(&mut self, pid: Pid) -> Option<&mut JobStruct> {
        for job in self.jobvec.iter_mut().filter(|x| x.is_some()) {
            if job.as_ref().unwrap().pid == pid {
                return job.as_mut();
            }
        }
        None // No job found
    }

    /// Get a job by its PID
    pub fn getjob_jid(&mut self, jid: Jid) -> Option<&mut JobStruct> {
        for job in self.jobvec.iter_mut().filter(|x| x.is_some()) {
            if job.as_ref().unwrap().jid == jid {
                return job.as_mut();
            }
        }
        None // No job found
    }

    /// Map process ID to job ID
    pub fn pid2jid(&self, pid: Pid) -> Option<Jid> {
        for job in self.jobvec.iter().flat_map(|x| x.iter()) {
            if job.pid == pid {
                return Some(job.jid);
            }
        }
        None
    }

    pub fn jid2pid(&self, jid: Jid) -> Option<Pid> {
        for job in self.jobvec.iter().flat_map(|x| x.iter()) {
            if job.jid == jid {
                return Some(job.pid);
            }
        }
        None
    }

    // List the jerbs
    pub fn listjobs(&self) -> Result<String, JobListError> {
        let mut output = String::new();
        for job in self.jobvec.iter().flat_map(|x| x.iter()) {
            let jid = job.jid;
            let pid = job.pid;
            let state = match job.state {
                JobState::BG => "Running",
                JobState::FG => "Foreground",
                JobState::Stop => "Stopped",
            };
            let jobstr = format!(
                "[{}] ({}) {} {}",
                jid,
                pid,
                state,
                (&job.cmdline[..]).trim()
            );
            output = format!("{}{}\n", output, jobstr);
        }
        Ok(output)
    }
}
