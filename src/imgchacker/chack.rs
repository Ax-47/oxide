
use url::{Url};
pub fn image_chacker(img:&std::string::String)->bool{
    if Url::parse(img.as_str()).is_ok() {
        return true;
    }
    let list=vec!["!img(guild)","!img(user)","!img(none)"];
    list.contains(&img.as_str())


}