Rust Minishell
==============

An implementation of the [minishell project](http://www.cs.utexas.edu/%7Eans/classes/cs439/projects/shell_project/shell.html)
in Rust. Conforms to the behavior of the mshref binary, even though its behavior
w.r.t. external `SIGCONT` is arguably broken.

Improvement points:
  - Joblist API is convoluted and difficult to use
  - Access to global joblist is unsafe
  - Potentially make a signal lock, i.e. a lock that works by blocking all
    signals and use that to handle the unsafe issue
  - Potentially wrap joblist in a safe API abstraction?
  - Error handling should be cleaned up and abstracted properly instead of being
    made ad-hoc. Potentially switch to using `dyn error` instead of explicit
    enums in some cases?
