use wapcaplet::ll::lwc_string;
use hl::*;

// Based off of libcss's examples/example1.c
mod example1 {
    #[test]
    fn run() {
        let data = "h1 { color: red }\
                    h4 { color: #321 }\
                    h4, h5 { color: #123456 }";

        /*let params: CssStylesheetParams<(), (), (), ()> = CssStylesheetParams {
            params_version: CssStylesheetParamsVersion1,
            level: CssLevel21,
            charset: ~"UTF-8",
            url: ~"foo",
            title: ~"foo",
            allow_quirks: false,
            inline_style: false,
            resolve: Some(resolve_url),
            resolve_pw: None,
            import: None,
            import_pw: None,
            color: None,
            color_pw: None,
            font: None,
            font_pw: None
        };*/
    }

    fn resolve_url(pw: &(), base: &str, rel: &lwc_string, abs: & &lwc_string) -> CssError {
        CssOk
    }
}
