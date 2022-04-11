#[macro_export]
macro_rules! prettify_output {
    ($tag:ty,$color:ident,$highlight:ident, $block:block) => {
        println!(
            "{} {} ",
            stringify!($tag).$highlight(),
            "START:-----------------------------------".$color()
        );

        $block

        println!(
            "{} {}",
            "----------------------------------- END:".$color(),
            stringify!($tag).$highlight()
        );
    };
}
#[macro_export]
macro_rules! color_value {
    ($colorme:expr,$fmt_string:expr,$color:ident) => {
        format!($fmt_string, $colorme).$color()
    };
}
