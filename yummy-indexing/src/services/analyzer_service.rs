use crate::common::*;

/* 초성 리스트 */
const CHOSUNG_LIST: [char; 19] = [
    'ㄱ', 'ㄲ', 'ㄴ', 'ㄷ', 'ㄸ', 'ㄹ', 'ㅁ', 'ㅂ', 'ㅃ', 'ㅅ', 'ㅆ', 'ㅇ', 'ㅈ', 'ㅉ', 'ㅊ', 'ㅋ',
    'ㅌ', 'ㅍ', 'ㅎ',
];

pub trait AnalyzerService {
    //fn extract_chosung_list(&self, input: &[String]) -> Vec<String>;
    fn extract_chosung(&self, input: &str) -> String;
}

#[derive(Debug, new)]
pub struct AnalyzerServicePub;

impl AnalyzerService for AnalyzerServicePub {
    // #[doc = ""]
    // fn extract_chosung_list(&self, input: &[String]) -> Vec<String> {
    // }

    #[doc = ""]
    fn extract_chosung(&self, input: &str) -> String {
        input
            .chars()
            .filter_map(|ch| {
                if ('가'..='힣').contains(&ch) {
                    let uni_val: i32 = ch as i32 - 0xAC00;
                    let chosung_index: usize = (uni_val / (21 * 28)) as usize;
                    CHOSUNG_LIST.get(chosung_index).copied()
                } else if ch.is_ascii_alphabetic() || ch.is_ascii_digit() {
                    /* 영문,숫자는 그대로 유지 */
                    Some(ch)
                } else {
                    /* 기타 문자는 제거 */
                    None
                }
            })
            .collect()
    }
}
