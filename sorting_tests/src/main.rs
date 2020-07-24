use std::sync::mpsc::{channel, Sender};
use std::{thread, time};

fn main() {
    /*
    let sample = vec![
        255, 22, 134, 54, 258, 92, 140, 38, 266, 116, 257, 169, 232, 93, 13, 265, 70, 47, 290, 71,
        197, 182, 265, 223, 300, 36, 67, 16, 119, 96, 214, 46, 281, 205, 67, 244, 130, 99, 206, 71,
        129, 150, 29, 82, 239, 171, 74, 293, 40, 54, 254, 162, 85, 104, 193, 151, 36, 17, 170, 119,
        242, 12, 229, 228, 154, 86, 11, 108, 251, 114, 180, 145, 13, 97, 177, 220, 137, 281, 75,
        73, 262, 64, 122, 77, 266, 70, 134, 299, 294, 153, 36, 79, 106, 116, 110, 196, 230, 27,
        201, 175,
    ];
    */

    let sample = vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
    //let sample = vec![10, 4, 71, 349, 218, 38, 23, 5,6, 23, 18, 40,89, 291, 3, 98];

    // Starting the sorting thread
    let (sender, receiver) = channel();
    start_quick(sample.clone(), (sample.len() - 1), sender);

    loop {
        // Received a value
        if let Ok(received) = receiver.recv() {
            println!("{:?}", received);
        } else {
            // No values left to receive
            break;
        }
    }
}

fn start_quick(mut data: Vec<i32>, len: usize, sender: Sender<Vec<i32>>) {
    thread::spawn(move || {
        //quick_sort(data, sender);
        quick_sort(&mut data, 0, len as i32, sender);
    });
}

fn quick_sort(array: &mut Vec<i32>, first: i32, last: i32, sender: Sender<Vec<i32>>) {
    if first < last {
        let pivot = first;
        let mut i = first;
        let mut j = last;

        while i < j {
            while array[i as usize] <= array[pivot as usize] && i < last {
                i += 1;
            }
            while array[j as usize] > array[pivot as usize] {
                j -= 1;
            }
            if i < j {
                array.swap(i as usize, j as usize);
                sender.send(array.clone()).unwrap();
                thread::sleep(time::Duration::from_millis(100));
            }
        }

        array.swap(pivot as usize, j as usize);
        sender.send(array.clone()).unwrap();
        thread::sleep(time::Duration::from_millis(100));

        quick_sort(array, first, j - 1, sender.clone());
        quick_sort(array, j + 1, last, sender.clone());
    }
}

/*
fn partition(array: &mut Vec<i32>, left: i32, right: i32, sender: Sender<Vec<i32>>) -> i32 {
    let pivot = array[left as usize];
    let mut i = left;
    let mut j = right + 1;

    while i >= j {
        while {
            i += 1;
            i <= right && array[i as usize] <= pivot
        } {
            i += 1;
        }
        while {
            j -= 1;
            array[j as usize] > pivot
        } {
            j -= 1;
        }

        if i >= j {
            break;
        }

        array.swap(i as usize, j as usize);
    }
    println!("J{} LEFT{}", j, left);
    array.swap(j as usize, left as usize);

    j
}

fn quick_sort(mut array: &mut Vec<i32>, left: i32, right: i32, sender: Sender<Vec<i32>>) {
    if left < right {
        let piv = partition(&mut array, left, right, sender.clone());
        quick_sort(&mut array, left, piv - 1, sender.clone());
        quick_sort(&mut array, piv + 1, right, sender.clone());
    }
    println!("ARRAY: {:?}", array);
}
*/

/*

fn partition(array: &mut Vec<i32>, sender: Sender<Vec<i32>>) -> usize {
    let mut i = 0;
    let right = array.len() - 1;

    for j in 0..right {
        if array[j] <= array[right] {
            array.swap(j, i);
            i += 1;
            sender.send(array.clone()).unwrap();
            thread::sleep(time::Duration::from_millis(100));
        }
    }

    array.swap(i, right);
    sender.send(array.clone()).unwrap();
    thread::sleep(time::Duration::from_millis(100));
    i
}

pub fn quick_sort(mut array: Vec<i32>, sender: Sender<Vec<i32>>) {
    if array.len() > 1 {
        let q = partition(&mut array, sender.clone());
        quick_sort(array[..q].to_vec(), sender.clone());
        quick_sort(array[q + 1..].to_vec(), sender.clone());
    }
    println!("ARRAY: {:?}", array);
}
 */
/*
fn start_bubble<T: 'static + PartialOrd + Copy + Send>(mut data: Vec<T>, sender: Sender<Vec<T>>) {
    thread::spawn(move || {
        bubble_sort(&mut data, sender);
    });
}

pub fn bubble_sort<'a, T: PartialOrd + Copy + Send>(array: &'a mut Vec<T>, sender: Sender<Vec<T>>) {
    for i in 0..array.len() {
        for j in 0..array.len() - i - 1 {
            if array[j + 1] < array[j] {
                array.swap(j, j + 1);
            }
        }
        sender.send(array.clone()).unwrap();
        thread::sleep(time::Duration::from_millis(10));
    }
    drop(sender);
}

fn start_shell(data: Vec<i32>, sender: Sender<Vec<i32>>) {
    thread::spawn(move || {
        shell_sort(data, sender);
    });
}

pub fn shell_sort(mut array: Vec<i32>, sender: Sender<Vec<i32>>) {
    let mut count_sublist = array.len() / 2;
    while count_sublist > 0 {
        for pos_start in 0..count_sublist {
            //sort_gap_insertation(&mut array, pos_start, count_sublist);
            for i in ((pos_start + count_sublist)..array.len()).step_by(count_sublist) {
                let val_current = array[i];
                let mut pos = i;

                while pos >= count_sublist && array[pos - count_sublist] > val_current {
                    array[pos] = array[pos - count_sublist];
                    pos = pos - count_sublist;
                }
                array[pos] = val_current;
                sender.send(array.clone()).unwrap();
                thread::sleep(time::Duration::from_millis(100));
            }
        }
        count_sublist /= 2;
    }
}
*/
