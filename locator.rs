use std::cmp::Ordering;
use std::collections::HashMap;

pub trait PriorityQueue<T: PartialOrd> {
    fn enqueue(&mut self, ele: T) -> ();
    fn dequeue(&mut self) -> Option<T>;
    fn peek(&self) -> Option<&T>;
}

/**
    An optional definition of a Node struct you may find useful
**/
struct Node<T> {
    priority: i32,
    data: T,
}

/** 
    These traits are implemented for Nodes to make them comparable 
**/
impl<T> PartialOrd for Node<T> {
    fn partial_cmp(&self, other: &Node<T>) -> Option<Ordering> {
        self.priority.partial_cmp(&other.priority)
    }
}

impl<T> PartialEq for Node<T> {
    fn eq(&self, other: &Node<T>) -> bool {
        self.priority == other.priority
    }
}

/** 
    You must implement the above trait for the vector type 
**/
impl<T: PartialOrd> PriorityQueue<T> for Vec<T> {
    /**
        This functions pushes a given element onto the queue and
        reorders the queue such that the min heap property holds.
        See the project specifications for more details on how this
        works.
    **/
    fn enqueue(&mut self, ele: T) -> () {

        self.push(ele);

        let mut new_elem_ind : usize = self.len() - 1;
        
        if new_elem_ind == 0 {
            
            return;
            
        } else if new_elem_ind == 1 {
            
            if self[0] > self[1] {
            
                self.swap(0, 1);

            }
            
        } else {
            
            while new_elem_ind > 0 && self[new_elem_ind] < self[(new_elem_ind - 1) / 2] {

                self.swap((new_elem_ind - 1) / 2, new_elem_ind);
                
                new_elem_ind = (new_elem_ind - 1) / 2;

            }
            
        }

    }

    /**
        This function removes the root element from the queue and
        reorders the queue such that it maintains the min heap
        property.  See the project specifications for more details.
        You should return the deleted element in the form of an option.
        Return None if the queue was initially empty, Some(T) otherwise.
    **/
    fn dequeue(&mut self) -> Option<T> {

        if self.len() == 0 {

            return None;

        }

        let last_elem = self.len() - 1;
        
        self.swap(0, last_elem);

        let root = self.remove(last_elem);
        let mut elem_ind = 0;

        loop {
        
            if (2 * elem_ind) + 1 >= self.len() || (2 * elem_ind) + 2 >= self.len() ||
                (self[(2 * elem_ind) + 1] > self[elem_ind] && self[(2 * elem_ind) + 2] > self[elem_ind]) {

                break;

            }

            let smallest_child_ind;

            if self[(2 * elem_ind) + 1] <= self[(2 * elem_ind) + 2] {

                smallest_child_ind = (2 * elem_ind) + 1;

            } else {

                smallest_child_ind = (2 * elem_ind) + 2;

            }
            
            self.swap(smallest_child_ind, elem_ind);

            if self[(2 * elem_ind) + 1] <= self[(2 * elem_ind) + 2] {

                elem_ind = (2 * elem_ind) + 1;

            } else {

                elem_ind = (2 * elem_ind) + 2;

            }

        }

        return Some(root);

    }

    /**
        This function returns the element that would be removed
        if dequeue were called on the queue.  There should be no
        mutations to the queue.  Return the element in the form
        of an option.  Return None if the queue is empty, Some(T)
        otherwise.
    **/
    fn peek(&self) -> Option<&T> {

        if self.len() == 0 {

            return None;

        }

        let first_elem = self.get(0);

        match first_elem {

            None => return None,
            Some(x) => {

                return Some(x);

            },

        }

    }
}

/**
    You must implement this function that computes the orthogonal
    distance between two coordinates.  Remember, orthogonal distance
    is not like Euclidean distance.  See the specifications for more
    details.
**/
pub fn distance(p1: (i32,i32), p2: (i32,i32)) -> i32 {
    
    let left_right : i32 = p1.0 - p2.0;
    let up_down : i32 = p1.1 - p2.1;

    left_right.abs() + up_down.abs()

}

/**
    You must implement this function that determines which enemy Stark
    should battle and their coordinates.  You are given two hashmaps for
    allies and enemies.  Each maps a name to their current coordinates.
    You can assume that the allies hashmap will always have a name
    called "Stark" included.  Return the name and coordinates of the enemy
    Stark will battle in the form of a 3-tuple.  See the specifications
    for more details on how to choose which enemy.
**/
pub fn target_locator<'a>(allies: &'a HashMap<&String, (i32,i32)>, enemies: &'a HashMap<&String, (i32,i32)>) -> (&'a str,i32,i32) {

    let mut clones = allies.clone();
    let (mut shortest, mut curr, mut dist) = (0, "", 0);

    for (key, value) in enemies {

        shortest = 10000;

        for (name, pos) in &clones {

            dist = distance(*value, *pos);

            if dist < shortest {

                curr = name;
                shortest = dist;

            }

        }

        if curr == "Stark" {

            return (key, value.0, value.1);

        } else {

            clones.remove(&curr.to_string());

        }

    }

    return ("Stark", 0, 0);

}