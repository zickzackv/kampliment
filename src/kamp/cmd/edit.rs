use super::Context;
use super::Error;

pub(crate) fn edit(ctx: Context, files: Vec<String>) -> Result<(), Error> {
    let mut buf = String::new();
    if files.is_empty() {
        buf.push_str("edit -scratch; ");
    } else {
        let names = files.iter().fold(String::new(), |mut buf, item| {
            if !item.starts_with("+") {
                buf.push_str("\n");
            }
            buf.push_str(item);
            buf
        });
        for name in names.split("\n").skip_while(|&s| s.is_empty()) {
            let mut edit = String::from("edit -existing ");
            for (i, item) in name.splitn(2, "+").enumerate() {
                match i {
                    0 => {
                        edit.push_str("'");
                        edit.push_str(item);
                        edit.push_str("'");
                    }
                    1 => item
                        .splitn(2, ":")
                        .take_while(|&s| s.parse::<i32>().is_ok())
                        .for_each(|n| {
                            edit.push_str(" ");
                            edit.push_str(&n.to_string());
                        }),
                    _ => unreachable!(),
                }
            }
            buf.push_str(&edit);
            buf.push_str("; ");
        }
    }
    buf.push_str("echo -to-file %opt{kamp_out}");
    if ctx.client.is_some() {
        ctx.send(&buf).map(|_| ())
    } else {
        ctx.connect(&buf)
    }
}
