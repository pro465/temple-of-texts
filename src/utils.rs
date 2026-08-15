// normal base: contains no "leading" (actually trailing in the Vec) 0's
// bijective base 256: can contain "leading" 0's.
//     0's function as a digit "256" in this base.

pub(crate) fn from_bijbase256(num: &mut Vec<u8>) {
    let mut carry = false;
    for i in num.iter_mut() {
        let t = carry;
        carry = *i == 0;
        if t {
            *i = i.wrapping_add(1);
        }
    }
    if carry {
        num.push(1);
    }
}

pub(crate) fn to_bijbase256(num: &mut Vec<u8>) {
    let mut carry = false;
    for i in num.iter_mut() {
        if carry {
            *i = i.wrapping_sub(1);
        }
        carry = *i == 0;
    }
    if carry {
        num.pop();
    }
}

const HOR_MARGIN: usize = 3;
const VER_MARGIN: usize = 1;
const COL_WIDTH: usize = 80;

fn repeated_push(s: &mut String, c: char, n: usize) {
    s.reserve(n);
    for _ in 0..n {
        s.push(c);
    }
}
/*
    print("╔"+"═"*(margin*2+width)+"╗")
    for i in range(margin): print("║"+" "*(margin*2+width)+"║")
    for l in chunks(t, width):
        print("║"+" "*margin+l+" "*margin+"║")
    for i in range(margin): print("║"+" "*(margin*2+width)+"║")
    print(""+"═"*(margin*2+width)+"")
*/
pub(crate) fn textbox(s: String) -> String {
    let mut res = String::from("╔");
    repeated_push(&mut res, '═', COL_WIDTH + HOR_MARGIN * 2);
    res.push_str("╗\n");
    for _ in 0..VER_MARGIN {
        res.push_str("║");
        repeated_push(&mut res, ' ', COL_WIDTH + HOR_MARGIN * 2);
        res.push_str("║\n");
    }

    let mut i = 0;
    
    for c in s.chars() {
        if i == 0 {
            res.push('║');
            repeated_push(&mut res, ' ', HOR_MARGIN);
        }

        res.push(c);
        
        if i == COL_WIDTH-1 {
            repeated_push(&mut res, ' ', HOR_MARGIN);
            res.push_str("║\n");
        }

        i+=1;
        i%=COL_WIDTH;
    }

    if i != 0 {
        repeated_push(&mut res, ' ', COL_WIDTH - i + HOR_MARGIN);
        res.push_str("║\n");
    }

    for _ in 0..VER_MARGIN {
        res.push_str("║");
        repeated_push(&mut res, ' ', COL_WIDTH + HOR_MARGIN * 2);
        res.push_str("║\n");
    }
    
    res.push('╚');
    repeated_push(&mut res, '═', COL_WIDTH + HOR_MARGIN * 2);
    res.push_str("╝\n");

    res
}
