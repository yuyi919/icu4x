
import { ICU4XCodePointMapData8, ICU4XDataProvider, ICU4XLineSegmenter, ICU4XSentenceBreakIteratorLatin1, ICU4XGraphemeClusterSegmenter } from './lib/index.mjs';

const text = "你可以看到\u2139\uFE0F绘制出来的emoji🌟与文本框中的样式一致。它也支持特殊的控制字符，如设置肤色👨\u{1F3FD}或将多个emoji拼合在一起的样式。\u{1F469}\u200D\u{1F469}\u200D\u{1F467}，就像这样。"
const provider = ICU4XDataProvider.create_compiled();
const segmenter = ICU4XLineSegmenter.create_lstm_with_options_v1(provider, {})

const segments = [];

let utf8Index = 0;
let utf16Index = 0;
const iter8 = segmenter.segment_utf8(text);
while (true) {
    const next = iter8.next();

    if (next === -1) {
        segments.push(text.slice(utf16Index));
        break;
    } else {
        const oldUtf16Index = utf16Index;
        while (utf8Index < next) {
            const codePoint = text.codePointAt(utf16Index);
            const utf8Len = (codePoint <= 0x7F) ? 1
                : (codePoint <= 0x7FF) ? 2
                    : (codePoint <= 0xFFFF) ? 3
                        : 4;
            const utf16Len = (codePoint <= 0xFFFF) ? 1
                : 2;
            utf8Index += utf8Len;
            utf16Index += utf16Len;
        }
        segments.push(text.slice(oldUtf16Index, utf16Index));
    }
}
console.log(segments)

