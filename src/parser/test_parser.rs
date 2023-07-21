#[cfg(test)]
mod tests {
    use crate::parser::CSVParser;

    #[test]
    fn line_parse_1() {
        let file = "./demo-package/messages/c151326093482262528";
        let parser: CSVParser = CSVParser::new(file.to_string());
        let line = String::from("000000000000000001,2021-02-03 09:52:40.224000+00:00,thx,");
        let res = parser.parse_line(line);

        assert!(res.is_ok());

        let msg = res.expect("Checked");

        assert_eq!(msg.id, String::from("000000000000000001"));
        assert_eq!(msg.channel_id, String::from("151326093482262528"));
    }

    #[test]
    fn line_parse_2() {
        let file = "./demo-package/messages/c151326093482262529";
        let parser: CSVParser = CSVParser::new(file.to_string());
        let line = String::from(
            "000000000000000002,\
            2021-02-02 13:15:02.813000+00:00,\
            Or just a better way,\
            https://cdn.discordapp.com/attachments/151326093482262528/806150690740240434/unknown.png");
        let res = parser.parse_line(line);

        assert!(res.is_ok());

        let msg = res.expect("Checked");

        assert_eq!(msg.id, String::from("000000000000000002"));
        assert_eq!(msg.channel_id, String::from("151326093482262529"));
    }

    #[test]
    fn file_parse() {
        let file = "./demo-package/messages/c151326093482262528";
        let parser: CSVParser = CSVParser::new(file.to_string());
        let msgs = parser.parse_file();

        assert_eq!(msgs.len(), 4);
        let msg = msgs.get(0).unwrap();
        assert_eq!(msg.id, "806465486845509673");
        assert_eq!(msg.channel_id, "151326093482262528");

        let msg = msgs.get(1).unwrap();
        assert_eq!(msg.id, "806464774471942154");
        assert_eq!(msg.channel_id, "151326093482262528");

        let msg = msgs.get(2).unwrap();
        assert_eq!(msg.id, "806462151350288404");
        assert_eq!(msg.channel_id, "151326093482262528");

        let msg = msgs.get(3).unwrap();
        assert_eq!(msg.id, "806462131008176208");
        assert_eq!(msg.channel_id, "151326093482262528");
    }
}