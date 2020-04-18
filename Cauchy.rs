use std::fmt;
use std::ops;
use std::cmp;

pub struct CauchyList {
    pub p: i32,
    pub content: Vec<i32>
}

impl CauchyList {
    fn pinj(mut inp1: Vec<i32>, inp2: Vec<i32>) -> Vec<i32> {
        let a = inp1.len();
        let b = inp2.len();
        if a > b || a ==b {
            return inp1;
        } else {
            for _ind2 in 0..(b-a) {
                inp1.push(0);
            }
            return inp1;
        }
    }
}

fn main()
{
    let mut vec = Vec::new();
    let mut vec1 = Vec::new();
    let mut vec2 = Vec::new();
    vec.push(17);
    vec.push(9);
    vec.push(22);
    vec.push(27);
    vec.push(28);
    vec.push(27);
    vec.push(15);
    vec.push(28);
    vec.push(24);
    vec.push(1);
    vec1.push(12);
    vec1.push(4);
    vec1.push(7);
    vec1.push(15);
    vec1.push(13);
    vec1.push(4);
    vec2.push(12);
    vec2.push(4);
    vec2.push(7);
    vec2.push(15);
    vec2.push(13);
    vec2.push(4);
    let a = CauchyList {p: 31, content:vec};
    let b = CauchyList {p: 31, content:vec1};
    let c = CauchyList {p: 31, content:vec2};
    println!("{:?}", a.eq(&c));
    println!("{:?}", a.eq(&b));
    println!("{:?}", b.eq(&c));
    //let t = b*5;
    //println!("{:?}", t.content);
    println!("{:?}", b==c);
    println!("{}", a*b+c*2);

}

impl cmp::PartialEq for CauchyList {
    fn eq(&self, other: &Self) -> bool {
        if other.p == self.p {
            let a = &self.content;
            let b = &other.content;
            let temp = a.iter().zip(b).filter(|&(a, b)| a == b).count();
            if temp == a.len() && temp == b.len(){
                return true;
            }
        }
        return false;
    }
}

impl ops::Add<CauchyList> for CauchyList {
    type Output = CauchyList;
    fn add(self, other: CauchyList) -> CauchyList {
        let mut a = self.content;
        let mut b = other.content;
        if a.len() < b.len(){
            a = CauchyList::pinj(a.clone(), b.clone());
        } else{
            b = CauchyList::pinj(b.clone(), a.clone());
        }
        for ind in 0..a.len() {
            a[ind] = (a[ind] + b[ind]) % (&self.p)
        }
        return CauchyList {p: self.p, content: a};
    }
}

impl ops::Sub<CauchyList> for CauchyList {
    type Output = CauchyList;
    fn sub(self, other: CauchyList) -> CauchyList {
        let mut a = self.content;
        let mut b = other.content;
        if a.len() < b.len(){
            a = CauchyList::pinj(a.clone(), b.clone());
        } else{
            b = CauchyList::pinj(b.clone(), a.clone());
        }
        for ind in 0..a.len() {
            a[ind] = (a[ind] - b[ind]) % (&self.p)
        }
        return CauchyList {p: self.p, content: a};
    }
}

impl ops::Mul<CauchyList> for CauchyList {
    type Output = CauchyList;
    fn mul(self, other: CauchyList) -> CauchyList {
        let mut vec = Vec::new();
        let mut a = self.content;
        let mut b = other.content;
        let s1 = a.clone().len();
        let s2 = b.clone().len();
        if a.len() < b.len(){
            a = CauchyList::pinj(a.clone(), b.clone());
        } else{
            b = CauchyList::pinj(b.clone(), a.clone());
        }
        let mut temp = 0;
        for ind in 0..(s1+s2-1){
            for ind2 in 0..ind+1{
                if ind2 < a.len() && (ind - ind2) < b.len(){
                temp = temp + a[ind2]*b[ind - ind2];
                }
            }
            vec.push(temp % (self.p));
            temp = 0;
        }
        return CauchyList {p: self.p, content: vec};
    }
}

impl ops::Mul<i32> for CauchyList {
    type Output = CauchyList;
    fn mul(self, other: i32) -> CauchyList {
        let mut vec = Vec::new();
        let a = self.content;
        for i in 0..a.len(){
            vec.push((a[i]*other) % self.p);
        }
        return CauchyList {p: self.p, content: vec};
    }
}

impl fmt::Display for CauchyList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "p:{} \nlength:{} \ncontent{:?}", self.p, self.content.len(), self.content)
    }
}
