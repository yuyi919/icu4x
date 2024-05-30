// @ts-check
import { ICU4XDataProvider, ICU4XScriptWithExtensions, ICU4XCodePointSetData, ICU4XCodePointMapData16, ICU4XCodePointMapData8, ICU4XPropertyValueNameToEnumMapper } from './lib/index.mjs';
import asserts from "assert/strict"

const provider = ICU4XDataProvider.create_compiled()
// const scripts = ICU4XScriptWithExtensions.create(provider).as_borrowed()//.has_script()
// ICU4XPropertyValueNameToEnumMapper.load_line_break
// ICU4XCodePointSetData.load_grapheme_link()
// ICU4XCodePointMapData16
const LineBreak = ICU4XCodePointMapData8.load_line_break(provider)
// const LBS = ICU4XPropertyValueNameToEnumMapper.load_line_break(provider)
provider.d
const AL = 2
const BA = 4
const CM = 9
const RI = 39

asserts.equal(LineBreak.get32(0x3000), BA)
asserts.equal(LineBreak.get32(0), CM)
asserts.equal(LineBreak.get32(0x1F1EA), RI)
asserts.equal(LineBreak.get32(0x1F532), AL)
asserts.equal(LineBreak.get32(0xA9), AL)

// asserts.equal((LBData.get32(0x1F532))->toBe(AL))

console.log(AL, BA, CM, RI)