/// result example 123 -> 202cb962ac59075b964b07152d234b70
pub fn md5(input: String) -> String {
    let digest = md5::compute(input);
    format!("{:x}", digest)
}
