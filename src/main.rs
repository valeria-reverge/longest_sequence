
fn get_subsequence_with_greater_last_value(values: &[i32], value_indices: &Vec<usize>, key_value_ind:usize) ->usize
{
    let mut start: usize = 0;
    let mut end: usize = value_indices.len()-1;
    while start < end
    {
        let mid:usize = (start+end)/2;
        if values[value_indices[mid]] == values[key_value_ind]
        {
            return mid;
        }
        else if values[value_indices[mid]] < values[key_value_ind]
        {
            start = mid+1;
        }
        else
        {
            end = mid-1;
        }
    }
    return end
}

fn print_sequence( values: &[i32], previous_indices: &Vec<isize>, ind:usize )
{
    if previous_indices[ind]<0
    {
        print!("{} ",values[ind]);
        return;
    }
    print_sequence( values, previous_indices, previous_indices[ind] as usize);
    print!("{} ",values[ind]);
}

fn find_longest_increasing_sequence( values: &[i32])
{
    if values.len()==0
    {
        print!("Array is empty, no increasing sequence.\n");
        return;
    }

    let mut subsequence_last_value_index = vec![0; 1];
    let mut previous_indices:Vec<isize> = vec![-1; values.len()];
 
    for i in 1..values.len()
    {
        let last_ind = subsequence_last_value_index.len()-1;
        if values[i] <= values[subsequence_last_value_index[0]]
        {
            subsequence_last_value_index[0] = i;
        }
        else if values[i] > values[subsequence_last_value_index[last_ind]]
        {
            previous_indices[i] = subsequence_last_value_index[last_ind] as isize;
            subsequence_last_value_index.push(i);
        }
        else
        {
            let pos = get_subsequence_with_greater_last_value(values, &subsequence_last_value_index, i);
            previous_indices[i] = subsequence_last_value_index[pos - 1] as isize;
            subsequence_last_value_index[pos] = i;
        }
    }
 
    print!("Longest increasing sequence in array [ ");
    for i in 0..values.len()
    {
        print!("{} ",values[i]);
    }
    print!("] is [ ");
    print_sequence( values, &previous_indices, subsequence_last_value_index[subsequence_last_value_index.len()-1] );
    print!("]\n");
}

fn main()
{
    let sequence: [i32;3] = [1, 2, 3];
    find_longest_increasing_sequence( &sequence );
    let sequence: [i32;1] = [1];
    find_longest_increasing_sequence( &sequence );
    let sequence: [i32;3] = [3, 2, 1];
    find_longest_increasing_sequence( &sequence );
    let sequence: [i32;7] = [1, 2, 3, 1, 1, 1, 2];
    find_longest_increasing_sequence( &sequence );
    let sequence: [i32;7] = [1, 2, 1, 0, 1, 2, 3];
    find_longest_increasing_sequence( &sequence );
    let sequence: [i32;5] = [1, 2, 1, 0, 5 ];
    find_longest_increasing_sequence( &sequence );
    let sequence: [i32;0] = [];
    find_longest_increasing_sequence( &sequence );
    let sequence: [i32;10] = [30, 40, 1, 1, 2, 4, 5, 8, 9, 10];
    find_longest_increasing_sequence( &sequence );
}
