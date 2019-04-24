/// @number 929
/// @title Unique Email Addresses
/// @url https://leetcode.com/problems/unique-email-addresses
/// @difficulty easy


struct Solution();

impl Solution {
    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
        let mut real_emails = emails.into_iter().map(|email| {
            let email_split: Vec<&str> = email.split('@').collect();
            let name_slit: Vec<&str> = email_split[0].split('+').collect();
            let real_name = name_slit[0].replace(".", "");
            format!("{}@{}", real_name, email_split[1])
        }).collect::<Vec<String>>();
        println!("{:?}", real_emails);
        real_emails.sort();
        // dedup can only be used for sorted data.
        real_emails.dedup_by(|a, b| a == b);

        real_emails.len() as i32
    }
}


#[test]
fn test() {
    assert_eq!(Solution::num_unique_emails(vec!["a@a.com".to_string(), "a+a@a.com".to_string()]), 1);

    assert_eq!(Solution::num_unique_emails(vec![
        "fg.r.u.uzj+o.pw@kziczvh.com".to_string(),
        "r.cyo.g+d.h+b.ja@tgsg.z.com".to_string(),
        "fg.r.u.uzj+o.f.d@kziczvh.com".to_string(),
        "r.cyo.g+ng.r.iq@tgsg.z.com".to_string(),
        "fg.r.u.uzj+lp.k@kziczvh.com".to_string(),
        "r.cyo.g+n.h.e+n.g@tgsg.z.com".to_string(),
        "fg.r.u.uzj+k+p.j@kziczvh.com".to_string(),
        "fg.r.u.uzj+w.y+b@kziczvh.com".to_string(),
        "r.cyo.g+x+d.c+f.t@tgsg.z.com".to_string(),
        "r.cyo.g+x+t.y.l.i@tgsg.z.com".to_string(),
        "r.cyo.g+brxxi@tgsg.z.com".to_string(),
        "r.cyo.g+z+dr.k.u@tgsg.z.com".to_string(),
        "r.cyo.g+d+l.c.n+g@tgsg.z.com".to_string(),
        "fg.r.u.uzj+vq.o@kziczvh.com".to_string(),
        "fg.r.u.uzj+uzq@kziczvh.com".to_string(),
        "fg.r.u.uzj+mvz@kziczvh.com".to_string(),
        "fg.r.u.uzj+taj@kziczvh.com".to_string(),
        "fg.r.u.uzj+fek@kziczvh.com".to_string()
    ]), 2);
}