fn main() {
    println!("{}", int_to_roman(3999));
}

fn int_to_roman(num: i32) -> String {
    let mut m_num = num;
    let mut dis = 1000;
    let mut grd = 0;
    
    let mut result = String::from("");

    let rom_list = vec!["M", "D", "C", "L", "X", "V", "I"];

    for (idx, rom) in rom_list.iter().enumerate() {
        for _ in 0..(m_num / dis) {
            result += rom;
        }

        m_num -= (m_num / dis) * dis;

        if (idx + 1) % 2 == 1 {
            grd = dis / 10;
        } 

        if m_num + grd >= dis {
            result += rom_list[idx+1+((idx+1)/2)];
            result += rom;

            m_num -= (m_num / grd) * grd ;
        }

        if (idx + 1) % 2 == 1 {
            grd = dis / 10;
            dis /= 2;
        } else {
            dis /= 5;
        }

        println!("{}", m_num);
        println!("{}", result);

    }

    result

}
