use std::env;
mod sort;

fn ft_atoi(s:String) -> Result<i32, String>
{
    let mut is_first = true;
    let mut sign = 1;
    let mut res: i32 = 0;

    for c in s.chars()
    {
        if is_first && c == '-'
        {
            sign = -1;
            is_first = false;
        }
        else if c.is_digit(10)
        {
            res *= 10;
            res += c.to_digit(10).unwrap() as i32;
        }
        else {
            return Err("ddd".to_string());
        }

    }
    Ok(res * sign)
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    let mut v:Vec<i32> = args.into_iter()
        .skip(1)
        .map(|s| {
            match ft_atoi(s) {
                Ok(number) => number,
                Err(_) => 0
            }
        })
        .collect();
    sort::ft_quick(&mut v[..]);
    println!("{:?}", v);
    Ok(())
}
