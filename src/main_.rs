
fn find_longest_increasing_sequence( integer_values: &[i32])
{
    if integer_values.len()==0
    {
        print!("Array is empty, no increasing sequence.\n");
        return;
    }
    let mut longest_sequence_start=0;
    let mut longest_sequence_end=0;
    let mut sequence_start=0;
    let mut sequence_end=0;
    for i in 1..integer_values.len()
    {
        if integer_values[sequence_end]<integer_values[i]
        {
            sequence_end=i;    
        }
        else 
        {
            if longest_sequence_end-longest_sequence_start<sequence_end-sequence_start 
            {
                longest_sequence_start=sequence_start;
                longest_sequence_end=sequence_end;   
            }
            sequence_start=i;
            sequence_end=i;    
        }
    }
    if longest_sequence_end-longest_sequence_start<sequence_end-sequence_start 
    {
        longest_sequence_start=sequence_start;
        longest_sequence_end=sequence_end;   
    }

    print!("Longest increasing sequence in array [");
    for i in 0..integer_values.len()-1
    {
        print!("{},",integer_values[i]);
    }
    print!("{}] is [",integer_values[integer_values.len()-1]);
    for i in longest_sequence_start..longest_sequence_end
    {
        print!("{},",integer_values[i]);
    }
    print!("{}]\n",integer_values[longest_sequence_end]);
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
    let sequence: [i32;0] = [];
    find_longest_increasing_sequence( &sequence );
    let sequence: [i32;6] = [50, 3, 10, 7, 40, 80];
    find_longest_increasing_sequence( &sequence );
    }
