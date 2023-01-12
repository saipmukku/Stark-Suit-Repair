use std::convert::TryInto;

/**
    Returns the sum 1 + 2 + ... + n
    If n is less than 0, return -1
**/
pub fn gauss(n: i32) -> i32 {

    if n < 0 {
        
        return -1;
        
    }
    
    let mut cum : i32 = 0;
    let mut num : i32 = 0;
    
    while num != n {
        
        cum += n - num;
        num += 1;
        
    }
    
    cum

}

/**
    Returns the number of elements in the list that 
    are in the range [s,e]
**/
pub fn in_range(ls: &[i32], s: i32, e: i32) -> i32 {
    
    let mut counter : i32 = 0;
    
    for element in ls {

        if element >= &s && element <= &e {
        
            counter += 1;
            
        }
        
    }
    
    counter

}

/**
    Returns true if target is a subset of set, false otherwise

    Ex: [1,3,2] is a subset of [1,2,3,4,5]
**/
pub fn subset<T: PartialEq>(set: &[T], target: &[T]) -> bool {
    
    let mut len = 0;
    
    for first in target {
        
        for second in set {
            
            if first == second {
                
                len += 1;
                
            }
            
        }
        
    }
    
    if len == target.len() {
        
        return true;
        
    }
    
    false

}

/**
    Returns the mean of elements in ls. If the list is empty, return None
    It might be helpful to use the fold method of the Iterator trait
**/
pub fn mean(ls: &[f64]) -> Option<f64> {

    if ls.len() == 0 {
        
        return None;
        
    }

    let mut total : f64 = 0.0;
    let length : f64 = ls.len() as f64;
    
    for element in ls {
        
        total += element;
        
    }
    
    return Some(total / length);

}

/**
    Converts a binary number to decimal, where each bit is stored in order in the array
    
    Ex: to_decimal of [1,0,1,0] returns 10
**/
pub fn to_decimal(lst: &[i32]) -> i32 {

    let mut decimal_num = 0;
    let mut len_counter : u32 = (lst.len()).try_into().unwrap();
    const TWO : i32 = 2;
    
    for element in lst {
        
        if element == &1 {

            decimal_num += TWO.pow(len_counter - 1);

        }
        
        len_counter -= 1;
        
    }
    
    decimal_num

}

/**
    Decomposes an integer into its prime factors and returns them in a vector
    You can assume factorize will never be passed anything less than 2

    Ex: factorize of 36 should return [2,2,3,3] since 36 = 2 * 2 * 3 * 3
**/
pub fn factorize(n: u32) -> Vec<u32> {
    
    let mut vector : Vec<u32> = Vec::new();
    let mut num = n.clone();
    
    while num != 1 {
    
        let mut modifier = 2;
        
        while num % modifier != 0 && modifier <= num {
            
            modifier += 1;
            
        }
        
        num /= modifier;
        vector.push(modifier);
        
    }
    
    if num > 1 {
    
        vector.push(num);
    
    }
    
    vector

}

/** 
    Takes all of the elements of the given slice and creates a new vector.
    The new vector takes all the elements of the original and rotates them, 
    so the first becomes the last, the second becomes first, and so on.
    
    EX: rotate [1,2,3,4] returns [2,3,4,1]
**/
pub fn rotate(lst: &[i32]) -> Vec<i32> {
    
    if lst.len() == 0 {
        
        return Vec::new();
        
    }
    
    let mut vector : Vec<i32> = Vec::from(lst);
    let temp : i32 = vector.remove(0);

    vector.push(temp);
    
    vector
    
}

/**
    Returns true if target is a subtring of s, false otherwise
    You should not use the contains function of the string library in your implementation
    
    Ex: "ace" is a substring of "rustacean"
**/
pub fn substr(s: &String, target: &str) -> bool {
    
    let len = s.len();
    let mut string = s.clone();
    let mut ind = 0;

    if s.as_str() == target {

        return true;

    }
    
    while ind < len {
    
        println!("{string}");

        if string.starts_with(target) {
            
            return true;
            
        }
        
        if string.len() == 0 {
            
            break;
            
        }
        
        string = string[1..].to_string();
        ind += 1;
        
    }
    
    false

}

/**
    Takes a string and returns the first longest substring of consecutive equal characters

    EX: longest_sequence of "ababbba" is Some("bbb")
    EX: longest_sequence of "aaabbb" is Some("aaa")
    EX: longest_sequence of "xyz" is Some("x")
    EX: longest_sequence of "" is None
**/
pub fn longest_sequence(s: &str) -> Option<&str> {
    
    if s == "" {
        
        return None;
        
    }

    let mut removed = false;
    let mut removed_ind = 0;
    let mut starting_ind = 0;
    let mut longest_len = 0;
    let mut copied_string = s.clone().to_string();
    
    while !(copied_string.is_empty()) {
        
        let mut curr_len = 1;
        let curr_char : char = copied_string.remove(0);
        
        removed_ind += 1;

        while copied_string.starts_with(curr_char) {
            
            curr_len += 1;
            copied_string.remove(0);
            removed_ind += 1;
            
        }
        
        println!("{} {}", curr_len, removed_ind);
        
        if curr_len > longest_len {
            
            longest_len = curr_len;
            
            if !removed {
                
                removed = true;
                
            } else {
                
                starting_ind = removed_ind - curr_len;
                
            }
            
        }

    }

    return Some(&s[starting_ind..starting_ind + longest_len]);

}