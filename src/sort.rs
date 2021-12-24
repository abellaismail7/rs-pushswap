
pub fn ft_quick(nums: &mut [i32])
{
    let len = nums.len();

    if len < 1
    {
        return ;
    }

    let last = nums[len - 1];
    let mut index = 0;
    let mut i= 0;

    while i < len
    {
        if nums[i] <= last
        {
            nums.swap(i, index);
            index += 1;
        }
        i += 1;
    }
    ft_quick(&mut nums[index..len]);
    index -= 1;
    ft_quick(&mut nums[0..index]);
}

pub fn ft_sort(nums:&mut Vec<i32>)
{
    let inds:Vec<i32> = nums
        .iter()
        .enumerate()
        .map(|(i, _)| i as i32)
        .collect();


    ft_quick(&mut nums[..]);
    for i in inds
    {
        println!("{}", i);
    }

}
