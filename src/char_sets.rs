// src/char_sets.rs

/*
In this section, you can add different character styles to your artwork,
 with the first letter being the strongest color and progressing to the last letter (the weakest).
*/

pub enum CharSet {
    Japanese,
    Korean,
    Block,
    Point,
    Chinese,
    MonoBlock
}

pub struct CharSets {
    pub ascii_chars: Vec<char>,
}

impl CharSets {

    pub fn new() -> Self {
        CharSets { ascii_chars: Vec::new() }
    }

    pub fn set_charset(&mut self, char_set: CharSet) {
        self.ascii_chars = match char_set {
            CharSet::Japanese => vec![
                'あ', 'い', 'う', 'え', 'お', 'か', 'き', 'く', 'け', 'こ',
                'さ', 'し', 'す', 'せ', 'そ', 'た', 'ち', 'つ', 'て', 'と',
                'な', 'に', 'ぬ', 'ね', 'の', 'は', 'ひ', 'ふ', 'へ', 'ほ',
                'ま', 'み', 'む', 'め', 'も', 'や', 'ゆ', 'よ', 'ら', 'り',
                'る', 'れ', 'ろ', 'わ', 'を', 'ん', 'ア', 'イ', 'ウ', 'エ', 'オ',
                'カ', 'キ', 'ク', 'ケ', 'コ', 'サ', 'シ', 'ス', 'セ', 'ソ',
                'タ', 'チ', 'ツ', 'テ', 'ト', 'ナ', 'ニ', 'ヌ', 'ネ', 'ノ',
                'ハ', 'ヒ', 'フ', 'ヘ', 'ホ', 'マ', 'ミ', 'ム', 'メ', 'モ',
                'ヤ', 'ユ', 'ヨ', 'ラ', 'リ', 'ル', 'レ', 'ロ', 'ワ', 'ヲ', 'ン'
            ],
            CharSet::Korean => vec![
                '가', '나', '다', '라', '마', '바', '사', '아', '자', '차',
                '카', '타', '파', '하', '거', '너', '더', '러', '머', '버',
                '서', '어', '저', '처', '커', '터', '퍼', '허', '길', '난',
                '다', '라', '마', '바', '사', '아', '자', '차', '카', '타'
            ],
            CharSet::Block => vec![
                '░', '▒', '▓', '█', '▒', '░', '▓', '█', '░', '▒',
                '▓', '█', '░', '▒', '▓', '█', '▒', '░', '▓',
                '█', '▒', '░', '░', '▒', '▓', '█', '▒', '░', '▓'
            ],
            CharSet::Point => vec![
                '.', ' '
            ],
            CharSet::Chinese => vec![
                '中', '国', '文', '字', '汉', '字', '你', '好', '我', '们', '学', '习', '程', '序', '编', '程'
            ],
            CharSet::MonoBlock => vec![
                '█', ' ' 
            ],
        }
    }
}
