use crate::core::prelude::*;
use crate::language::{EncodeType, PipeBase64Decode, PipeBase64Encode};
use base64::Engine;
use base64::engine::general_purpose;
use encoding::all::*;
use encoding::{DecoderTrap, Encoding};
use imap_types::utils::escape_byte_string;
use wp_model_core::model::{DataField, Value};

impl ValueProcessor for PipeBase64Encode {
    fn value_cacu(&self, in_val: DataField) -> DataField {
        match in_val.get_value() {
            Value::Chars(x) => {
                let encode = general_purpose::STANDARD.encode(x);
                DataField::from_chars(in_val.get_name().to_string(), encode)
            }
            _ => in_val,
        }
    }
}
impl ValueProcessor for PipeBase64Decode {
    fn value_cacu(&self, in_val: DataField) -> DataField {
        match in_val.get_value() {
            Value::Chars(x) => {
                if let Ok(code) = general_purpose::STANDARD.decode(x) {
                    let val_str = match self.encode {
                        EncodeType::Imap => escape_byte_string(code),
                        EncodeType::Utf8 => {
                            UTF_8.decode(&code, DecoderTrap::Ignore).unwrap_or_default()
                        }
                        EncodeType::Utf16le => UTF_16LE
                            .decode(&code, DecoderTrap::Ignore)
                            .unwrap_or_default(),
                        EncodeType::Utf16be => UTF_16BE
                            .decode(&code, DecoderTrap::Ignore)
                            .unwrap_or_default(),
                        EncodeType::EucJp => EUC_JP
                            .decode(&code, DecoderTrap::Ignore)
                            .unwrap_or_default(),
                        EncodeType::Windows31j => WINDOWS_31J
                            .decode(&code, DecoderTrap::Ignore)
                            .unwrap_or_default(),
                        EncodeType::Iso2022Jp => ISO_2022_JP
                            .decode(&code, DecoderTrap::Ignore)
                            .unwrap_or_default(),
                        EncodeType::Gbk => {
                            GBK.decode(&code, DecoderTrap::Ignore).unwrap_or_default()
                        }
                        EncodeType::Gb18030 => GB18030
                            .decode(&code, DecoderTrap::Ignore)
                            .unwrap_or_default(),
                        EncodeType::HZ => HZ.decode(&code, DecoderTrap::Ignore).unwrap_or_default(),
                        EncodeType::Big52003 => BIG5_2003
                            .decode(&code, DecoderTrap::Ignore)
                            .unwrap_or_default(),
                        EncodeType::MacCyrillic => MAC_CYRILLIC
                            .decode(&code, DecoderTrap::Ignore)
                            .unwrap_or_default(),
                        EncodeType::Windows874 => WINDOWS_874
                            .decode(&code, DecoderTrap::Ignore)
                            .unwrap_or_default(),
                        EncodeType::Windows949 => WINDOWS_949
                            .decode(&code, DecoderTrap::Ignore)
                            .unwrap_or_default(),
                        EncodeType::Windows1250 => WINDOWS_1250
                            .decode(&code, DecoderTrap::Ignore)
                            .unwrap_or_default(),
                        EncodeType::Windows1251 => WINDOWS_1251
                            .decode(&code, DecoderTrap::Ignore)
                            .unwrap_or_default(),
                        EncodeType::Windows1252 => WINDOWS_1252
                            .decode(&code, DecoderTrap::Ignore)
                            .unwrap_or_default(),
                        EncodeType::Windows1253 => WINDOWS_1253
                            .decode(&code, DecoderTrap::Ignore)
                            .unwrap_or_default(),
                        EncodeType::Windows1254 => WINDOWS_1254
                            .decode(&code, DecoderTrap::Ignore)
                            .unwrap_or_default(),
                        EncodeType::Windows1255 => WINDOWS_1255
                            .decode(&code, DecoderTrap::Ignore)
                            .unwrap_or_default(),
                        EncodeType::Windows1256 => WINDOWS_1256
                            .decode(&code, DecoderTrap::Ignore)
                            .unwrap_or_default(),
                        EncodeType::Windows1257 => WINDOWS_1257
                            .decode(&code, DecoderTrap::Ignore)
                            .unwrap_or_default(),
                        EncodeType::Windows1258 => WINDOWS_1258
                            .decode(&code, DecoderTrap::Ignore)
                            .unwrap_or_default(),
                        EncodeType::Ascii => {
                            ASCII.decode(&code, DecoderTrap::Ignore).unwrap_or_default()
                        }
                        EncodeType::Ibm866 => IBM866
                            .decode(&code, DecoderTrap::Ignore)
                            .unwrap_or_default(),
                        EncodeType::Iso88591 => ISO_8859_1
                            .decode(&code, DecoderTrap::Ignore)
                            .unwrap_or_default(),
                        EncodeType::Iso88592 => ISO_8859_2
                            .decode(&code, DecoderTrap::Ignore)
                            .unwrap_or_default(),
                        EncodeType::Iso88593 => ISO_8859_3
                            .decode(&code, DecoderTrap::Ignore)
                            .unwrap_or_default(),
                        EncodeType::Iso88594 => ISO_8859_4
                            .decode(&code, DecoderTrap::Ignore)
                            .unwrap_or_default(),
                        EncodeType::Iso88595 => ISO_8859_5
                            .decode(&code, DecoderTrap::Ignore)
                            .unwrap_or_default(),
                        EncodeType::Iso88596 => ISO_8859_6
                            .decode(&code, DecoderTrap::Ignore)
                            .unwrap_or_default(),
                        EncodeType::Iso88597 => ISO_8859_7
                            .decode(&code, DecoderTrap::Ignore)
                            .unwrap_or_default(),
                        EncodeType::Iso88598 => ISO_8859_8
                            .decode(&code, DecoderTrap::Ignore)
                            .unwrap_or_default(),
                        EncodeType::Iso885910 => ISO_8859_10
                            .decode(&code, DecoderTrap::Ignore)
                            .unwrap_or_default(),
                        EncodeType::Iso885913 => ISO_8859_13
                            .decode(&code, DecoderTrap::Ignore)
                            .unwrap_or_default(),
                        EncodeType::Iso885914 => ISO_8859_14
                            .decode(&code, DecoderTrap::Ignore)
                            .unwrap_or_default(),
                        EncodeType::Iso885915 => ISO_8859_15
                            .decode(&code, DecoderTrap::Ignore)
                            .unwrap_or_default(),
                        EncodeType::Iso885916 => ISO_8859_16
                            .decode(&code, DecoderTrap::Ignore)
                            .unwrap_or_default(),
                        EncodeType::Koi8R => KOI8_R
                            .decode(&code, DecoderTrap::Ignore)
                            .unwrap_or_default(),
                        EncodeType::Koi8U => KOI8_U
                            .decode(&code, DecoderTrap::Ignore)
                            .unwrap_or_default(),
                        EncodeType::MacRoman => MAC_ROMAN
                            .decode(&code, DecoderTrap::Ignore)
                            .unwrap_or_default(),
                    };

                    DataField::from_chars(in_val.get_name().to_string(), val_str)
                } else {
                    DataField::from_chars(in_val.get_name().to_string(), String::new())
                }
            }
            _ => in_val,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::DataTransformer;
    use crate::parser::oml_parse;
    use wp_data_model::cache::FieldQueryCache;
    use wp_model_core::model::{DataField, DataRecord};

    #[test]
    fn test_pipe_base64() {
        let cache = &mut FieldQueryCache::default();
        let data = vec![
            DataField::from_chars("A1", "hello1"),
            DataField::from_chars(
                "B2",
                "UE9TVCAvYWNjb3VudCBIVFRQLzEuMQ0KSG9zdDogZnRwLXh0by5lbmVyZ3ltb3N0LmNvbTo2MTIyMg0KVXNlci1BZ2VudDogTW96aWxsYS81LjAgKE1hY2ludG9zaDsgSW50ZWwgTWFjIE9TIFggMTBfMTVfNykgQXBwbGVXZWJLaXQvNTM3LjM2IChLSFRNTCwgbGlrZSBHZWNrbykgQ2hyb21lLzEwMS4wLjAuMCBTYWZhcmkvNTM3LjM2DQpDb250ZW50LUxlbmd0aDogMTE0DQpDb25uZWN0aW9uOiBjbG9zZQ0KQ29udGVudC1UeXBlOiBhcHBsaWNhdGlvbi94LXd3dy1mb3JtLXVybGVuY29kZWQNCkFjY2VwdC1FbmNvZGluZzogZ3ppcA0KDQo=",
            ),
            DataField::from_chars(
                "C3",
                "U1NILTIuMC1tb2Rfc2Z0cA0KAAADVAcUUhSdWEFUvYFEugJ7xA68OgAAAT1jdXJ2ZTI1NTE5LXNoYTI1NixjdXJ2ZTI1NTE5LXNoYTI1NkBsaWJzc2gub3JnLGVjZGgtc2hhMi1uaXN0cDUyMSxlY2RoLXNoYTItbmlzdHAzODQsZWNkaC1zaGEyLW5pc3RwMjU2LGRpZmZpZS1oZWxsbWFuLWdyb3VwMTgtc2hhNTEyLGRpZmZpZS1oZWxsbWFuLWdyb3VwMTYtc2hhNTEyLGRpZmZpZS1oZWxsbWFuLWdyb3VwMTQtc2hhMjU2LGRpZmZpZS1oZWxsbWFuLWdyb3VwLWV4Y2hhbmdlLXNoYTI1NixkaWZmaWUtaGVsbG1hbi1ncm91cC1leGNoYW5nZS1zaGExLGRpZmZpZS1oZWxsbWFuLWdyb3VwMTQtc2hhMSxyc2ExMDI0LXNoYTEsZXh0LWluZm8tcwAAAClyc2Etc2hhMi01MTIscnNhLXNoYTItMjU2LHNzaC1yc2Esc3NoLWRzcwAAAF9hZXMyNTYtY3RyLGFlczE5Mi1jdHIsYWVzMTI4LWN0cixhZXMyNTYtY2JjLGFlczE5Mi1jYmMsYWVzMTI4LWNiYyxjYXN0MTI4LWNiYywzZGVzLWN0ciwzZGVzLWNiYwAAAF9hZXMyNTYtY3RyLGFlczE5Mi1jdHIsYWVzMTI4LWN0cixhZXMyNTYtY2JjLGFlczE5Mi1jYmMsYWVzMTI4LWNiYyxjYXN0MTI4LWNiYywzZGVzLWN0ciwzZGVzLWNiYwAAAFtobWFjLXNoYTItMjU2LGhtYWMtc2hhMi01MTIsaG1hYy1zaGExLGhtYWMtc2hhMS05Nix1bWFjLTY0QG9wZW5zc2guY29tLHVtYWMtMTI4QG9wZW5zc2guY29tAAAAW2htYWMtc2hhMi0yNTYsaG1hYy1zaGEyLTUxMixobWFjLXNoYTEsaG1hYy1zaGExLTk2LHVtYWMtNjRAb3BlbnNzaC5jb20sdW1hYy0xMjhAb3BlbnNzaC5jb20AAAAaemxpYkBvcGVuc3NoLmNvbSx6bGliLG5vbmUAAAAaemxpYkBvcGVuc3NoLmNvbSx6bGliLG5vbmUAAAAAAAAAAAAAAAAAXuQ3JWG631Byb3RvY29sIG1pc21hdGNoLgo=",
            ),
        ];
        let src = DataRecord { items: data };

        let mut conf = r#"
        name : test
        ---
        X : chars =  pipe take(A1) | base64_en | base64_de() ;
        Y : chars =  pipe take(B2) | base64_de(Imap) ;
        Z : chars =  pipe take(C3) | base64_de(Imap) ;
         "#;
        let model = oml_parse(&mut conf).unwrap();

        let target = model.transform(src, cache);

        let expect = DataField::from_chars("X".to_string(), "hello1".to_string());
        assert_eq!(target.field("X"), Some(&expect));

        let expect = DataField::from_chars("Y".to_string(), r#"POST /account HTTP/1.1\r\nHost: ftp-xto.energymost.com:61222\r\nUser-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/101.0.0.0 Safari/537.36\r\nContent-Length: 114\r\nConnection: close\r\nContent-Type: application/x-www-form-urlencoded\r\nAccept-Encoding: gzip\r\n\r\n"#.to_string());
        assert_eq!(target.field("Y"), Some(&expect));

        let expect = DataField::from_chars("Z".to_string(), "SSH-2.0-mod_sftp\\r\\n\\x00\\x00\\x03T\\x07\\x14R\\x14\\x9dXAT\\xbd\\x81D\\xba\\x02{\\xc4\\x0e\\xbc:\\x00\\x00\\x01=curve25519-sha256,curve25519-sha256@libssh.org,ecdh-sha2-nistp521,ecdh-sha2-nistp384,ecdh-sha2-nistp256,diffie-hellman-group18-sha512,diffie-hellman-group16-sha512,diffie-hellman-group14-sha256,diffie-hellman-group-exchange-sha256,diffie-hellman-group-exchange-sha1,diffie-hellman-group14-sha1,rsa1024-sha1,ext-info-s\\x00\\x00\\x00)rsa-sha2-512,rsa-sha2-256,ssh-rsa,ssh-dss\\x00\\x00\\x00_aes256-ctr,aes192-ctr,aes128-ctr,aes256-cbc,aes192-cbc,aes128-cbc,cast128-cbc,3des-ctr,3des-cbc\\x00\\x00\\x00_aes256-ctr,aes192-ctr,aes128-ctr,aes256-cbc,aes192-cbc,aes128-cbc,cast128-cbc,3des-ctr,3des-cbc\\x00\\x00\\x00[hmac-sha2-256,hmac-sha2-512,hmac-sha1,hmac-sha1-96,umac-64@openssh.com,umac-128@openssh.com\\x00\\x00\\x00[hmac-sha2-256,hmac-sha2-512,hmac-sha1,hmac-sha1-96,umac-64@openssh.com,umac-128@openssh.com\\x00\\x00\\x00\\x1azlib@openssh.com,zlib,none\\x00\\x00\\x00\\x1azlib@openssh.com,zlib,none\\x00\\x00\\x00\\x00\\x00\\x00\\x00\\x00\\x00\\x00\\x00\\x00\\x00^\\xe47%a\\xba\\xdfProtocol mismatch.\\n".to_string());
        assert_eq!(target.field("Z"), Some(&expect));
    }
}
