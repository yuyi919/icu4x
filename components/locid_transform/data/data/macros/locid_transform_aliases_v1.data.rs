// @generated
/// Implement `DataProvider<AliasesV1Marker>` on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_locid_transform_aliases_v1 {
    ($ provider : ty) => {
        #[clippy::msrv = "1.67"]
        const _: () = <$provider>::MUST_USE_MAKE_PROVIDER_MACRO;
        #[clippy::msrv = "1.67"]
        impl $provider {
            #[doc(hidden)]
            pub const SINGLETON_LOCID_TRANSFORM_ALIASES_V1: &'static <icu::locid_transform::provider::AliasesV1Marker as icu_provider::DataMarker>::Yokeable = &icu::locid_transform::provider::AliasesV1 {
                language_variants: unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x13\0\0\0\0\0\x17\x000\0J\0c\0z\0\x92\0\xBB\0\xD1\0\xE8\0\xFF\0\x1B\x015\x01O\x01h\x01\x80\x01\x99\x01\xB3\x01\xCB\x01\x02\0\0\0\0\0\0\0\x08\0\0\0aa-saahossy\x02\0\0\0\0\0\0\0\n\0\0\0art-lojbanjbo\x02\0\0\0\0\0\0\0\x0B\0\0\0cel-gaulishxtg\x02\0\0\0\0\0\0\0\n\0\0\0hy-arevmdahyw\x02\0\0\0\0\0\0\0\t\0\0\0no-bokmalnb\x02\0\0\0\0\0\0\0\n\0\0\0no-nynorsknn\x02\0\0\0\0\0\0\0\x12\0\0\0und-hepburn-heplocund-alalc97\x02\0\0\0\0\0\0\0\x08\0\0\0zh-guoyuzh\x02\0\0\0\0\0\0\0\x08\0\0\0zh-hakkahak\x02\0\0\0\0\0\0\0\x08\0\0\0zh-xianghsn\x02\0\0\0\0\0\0\0\n\0\0\0und-aalandund-AX\x02\0\0\0\0\0\0\0\x0B\0\0\0und-arevelaund\x02\0\0\0\0\0\0\0\x0B\0\0\0und-arevmdaund\x02\0\0\0\0\0\0\0\n\0\0\0und-bokmalund\x02\0\0\0\0\0\0\0\t\0\0\0und-hakkaund\x02\0\0\0\0\0\0\0\n\0\0\0und-lojbanund\x02\0\0\0\0\0\0\0\x0B\0\0\0und-nynorskund\x02\0\0\0\0\0\0\0\t\0\0\0und-saahound\x02\0\0\0\0\0\0\0\t\0\0\0und-xiangund") },
                sgn_region: unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"BR\0CO\0DE\0DK\0ES\0FR\0GB\0GR\0IE\0IT\0JP\0MX\0NI\0NL\0NO\0PT\0SE\0US\0ZA\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"bzscsngsgdslsspfslbfigssisgisejslmfsncsdsensipsrswlasesfs") })
                },
                language_len2: unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"bhiniwjijwmoshtltw") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\t\0\0\0\0\0\x03\0\x05\0\x07\0\t\0\x0B\0\r\0\x14\0\x17\0bhoidheyijvrosr-Latnfilak") })
                },
                language_len3: unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"aamaarabkadpafragpaisajtajuakaalbalsamharaarbargarmasdasmaueavaaveaymayrayxazeazjbakbambaqbazbccbclbelbenbgmbhkbicbihbisbjdbjqbkbblgbodbosbrebtbbulburbxkbxrcatccqceschachechichuchvcjrckacldcmkcmncnrcorcoscoycqucrecwdcymczedafdandapdeudgodhddikdiqditdivdjldkldrhdrrdrwduddujdutdwldzoekkellelpemkengepoeskesteusewefaofasfatfijfinfrafrefryfucfulgavgazgbcgbogeogergfxggnggoggrgioglagleglggliglvgnogregrngtiguggujguvgyahathauhbshdnheahebherhimhinhmohrrhrvhunhyeibiiboiceidoiiiikeikuileillilwinaindipkislitaizijarjavjegjpnkalkankaskatkaukazkdvkgckgdkghkhkkhmkikkinkirkmrknckngknnkojkomkonkorkppkpvkrmktrkuakurkvskwqkxekxlkzhkzjkztlaklaolatlavlbklegliilimlinlitllolmmltzlubluglvsmacmahmalmaomarmaymegmgxmhrmkdmlgmltmnkmntmofmolmonmrimsamstmupmwdmwjmyamydmytnadnaunavnbfnblnbxncpndendonepnldnlnnlrnnonnsnnxnobnoonornpintsnxunyaociojgojioriormoryossounpanpatpbupcrperpesplipltpmcpmupnbpolporppapprprsprypuspuzquequzrmrrmyrohronrumrunrussagsansapscasccscrsglsinskkslksloslvsmdsmesmosnasnbsndsomsotspaspysqisrcsrdsrpsswsulsumsunswaswcsweswhtahtamtattduteltggtgktglthathcthwthxtibtidtietirtkktlwtmptnetnftontsftsntsottqtukturtwiuigukrumuunpuokurduzbuznvenvievolwelwgwwitwiwwlnwolxbaxhoxiaxkhxpexrqxsjxslybdyddyenyidyiyymaymtyoryosyuuzaizhazhozirzsmzulzyb") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x9E\x01\0\0\0\0\x03\0\x05\0\x07\0\t\0\x0B\0\x0E\0\x11\0\x14\0\x17\0\x19\0\x1B\0\x1D\0\x1F\0!\0#\0%\0'\0*\0,\0/\x001\x003\x005\x007\0:\0<\0>\0@\0B\0D\0G\0J\0M\0O\0Q\0T\0W\0Z\0]\0_\0b\0e\0h\0k\0m\0o\0q\0t\0v\0x\0{\0~\0\x80\0\x83\0\x85\0\x87\0\x89\0\x8B\0\x8D\0\x8F\0\x92\0\x95\0\x98\0\x9B\0\x9D\0\xA2\0\xA4\0\xA6\0\xA9\0\xAC\0\xAE\0\xB0\0\xB2\0\xB4\0\xB7\0\xB9\0\xBC\0\xBE\0\xC1\0\xC4\0\xC7\0\xCA\0\xCD\0\xCF\0\xD2\0\xD5\0\xD7\0\xDA\0\xDF\0\xE2\0\xE5\0\xE7\0\xEA\0\xEC\0\xEE\0\xF0\0\xF3\0\xF6\0\xF8\0\xFA\0\xFC\0\xFE\0\0\x01\x02\x01\x04\x01\x06\x01\x08\x01\n\x01\x0C\x01\x0E\x01\x10\x01\x12\x01\x14\x01\x16\x01\x19\x01\x1B\x01\x1E\x01!\x01#\x01%\x01(\x01+\x01.\x011\x014\x016\x018\x01:\x01=\x01?\x01B\x01D\x01F\x01I\x01K\x01M\x01P\x01S\x01U\x01W\x01^\x01a\x01d\x01f\x01h\x01k\x01m\x01o\x01r\x01t\x01v\x01x\x01{\x01}\x01\x7F\x01\x81\x01\x83\x01\x85\x01\x87\x01\x89\x01\x8C\x01\x8F\x01\x91\x01\x93\x01\x95\x01\x97\x01\x99\x01\x9C\x01\x9F\x01\xA1\x01\xA4\x01\xA6\x01\xA8\x01\xAA\x01\xAC\x01\xAE\x01\xB0\x01\xB2\x01\xB5\x01\xB8\x01\xBB\x01\xBE\x01\xC0\x01\xC2\x01\xC4\x01\xC6\x01\xC8\x01\xCA\x01\xCC\x01\xCE\x01\xD1\x01\xD4\x01\xD6\x01\xD8\x01\xDA\x01\xDD\x01\xDF\x01\xE2\x01\xE5\x01\xE7\x01\xE9\x01\xEC\x01\xEF\x01\xF2\x01\xF5\x01\xF8\x01\xFB\x01\xFE\x01\x01\x02\x03\x02\x05\x02\x07\x02\n\x02\r\x02\x10\x02\x12\x02\x14\x02\x16\x02\x19\x02\x1C\x02\x1E\x02 \x02\"\x02$\x02&\x02(\x02*\x02,\x02.\x020\x023\x026\x029\x02;\x02=\x02?\x02B\x02E\x02H\x02J\x02L\x02N\x02P\x02S\x02V\x02Y\x02\\\x02^\x02a\x02d\x02g\x02i\x02k\x02n\x02p\x02s\x02v\x02x\x02z\x02|\x02~\x02\x81\x02\x84\x02\x86\x02\x89\x02\x8C\x02\x8E\x02\x91\x02\x93\x02\x95\x02\x98\x02\x9B\x02\x9D\x02\x9F\x02\xA1\x02\xA3\x02\xA5\x02\xA7\x02\xA9\x02\xAB\x02\xAE\x02\xB0\x02\xB3\x02\xB5\x02\xB8\x02\xBA\x02\xBC\x02\xBE\x02\xC0\x02\xC3\x02\xC6\x02\xC9\x02\xCB\x02\xCD\x02\xD0\x02\xD3\x02\xD8\x02\xDB\x02\xDD\x02\xE0\x02\xE2\x02\xE4\x02\xE7\x02\xEA\x02\xEC\x02\xEE\x02\xF0\x02\xF2\x02\xF4\x02\xF6\x02\xF8\x02\xFB\x02\xFE\x02\0\x03\x02\x03\x05\x03\x07\x03\n\x03\x0C\x03\x0E\x03\x10\x03\x13\x03\x15\x03\x17\x03\x19\x03\x1C\x03\x1E\x03 \x03\"\x03$\x03'\x03)\x03+\x03-\x03/\x031\x034\x037\x039\x03;\x03@\x03B\x03D\x03F\x03H\x03J\x03M\x03O\x03R\x03T\x03W\x03Y\x03\\\x03_\x03b\x03d\x03g\x03j\x03l\x03o\x03r\x03u\x03x\x03}\x03\x7F\x03\x82\x03\x84\x03\x86\x03\x89\x03\x8B\x03\x8D\x03\x8F\x03\x91\x03\x93\x03\x96\x03\x99\x03\x9C\x03\x9E\x03\xA0\x03\xA2\x03\xA4\x03\xA6\x03\xA8\x03\xAA\x03\xAD\x03\xB0\x03\xB3\x03\xB5\x03\xB7\x03\xBA\x03\xBC\x03\xBF\x03\xC2\x03\xC5\x03\xC8\x03\xCB\x03\xCE\x03\xD1\x03\xD3\x03\xD6\x03\xD8\x03\xDB\x03\xDE\x03\xE1\x03\xE3\x03\xE6\x03\xE9\x03\xEC\x03\xEE\x03\xF0\x03\xF3\x03\xF5\x03\xF7\x03aasaaabdzafapfamiaebjrbaksqsqamararanhysnzasktzavaeayaynunazazbabmeunvobalbikbebnbcgfblbirbhobidrlbzcebkibabobsbrbebbgmyluybuacarkicschcezhcucvmomcmrsyrxchzhsr-MEkwcopijquhcrcrcycsdnjdanjzdedoimwrdinzzadifdvdzeaqdmnkzkfa-AFuthdwunldbtdzetelamqmaneneoiketeueefofaakfjfifrfrfyffffdevomwnygrbkadevajgvresggtuaougdgaglkzkgvgonelgnnycgnguduzgbahthasr-Latnhaihmnhehzsrxhihojalhrhuhyopaigisioiiiuiuieilmgaliaidikisitezajgkjvoybjaklknkskakrkkzkdtdfncqkmlmnkmkirwkykukrkgkokkwvkvkgkojkmkvbmfdtpkjkugdjyamtvdkrudgldtpdtpksplolalvbncenlraqlilnltngtrmxlblulglvmkmhmlmimrmscirjbkchmmkmgmtmanwnnxntromnmimsmryrajdmwvajmyaogmryxnynanvnrunrekckdzndngnenlazdnrknnnbrngvnbdtdnonepijbppnyocojojoromorosvajpakxrpsadxfafapimghuwphrlahplptbfylcqfa-AFprtpspubququemxromrmrorornrusgsaaqthlesrhrisksioybskskslkmbsesmsnibasdsostesklnsqscscsrsssgdulwsuswsw-CDsvswtytattdtptebjptgfilthtpoolaoybboitdrastitwmweotyjkakfa-AFtotajtntstmhtktrakugukdelwroemauruzuzvevivocywgbnolnwowawocaxxhacnwawkpedmwsujdenrkiyiynqyiyrmlrrmtmyozomyugzapzazhscvmszuza") })
                },
                language: zerovec::VarZeroVec::new(),
                script: unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"Qaai") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"Zinh") })
                },
                region_alpha: unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"BUCTDDDYFXHVJTMINHNQPUPZQURHTPUKVDWKYDZR") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"MM\0KI\0DE\0BJ\0FR\0BF\0UM\0UM\0VU\0AQ\0UM\0PA\0EU\0ZW\0TL\0GB\0VN\0UM\0YE\0CD\0") })
                },
                region_num: unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"004008010012016020024028031032036040044048050051052056060064068070072074076084086090092096100104108112116120124132136140144148152156158162166170174175178180184188191192196203204208212214218222226230231232233234238239242246248249250254258260262266268270275276278280288292296300304308312316320324328332334336340344348352356360364368372376380384388392398400404408410414417418422426428430434438440442446450454458462466470474478480484492496498499500504508512516520524528531533534535540548554558562566570574578580581583584585586591598600604608612616620624626630634638642643646652654659660662663666670674678682686688690694702703704705706710716720724728729732736740744748752756760762764768772776780784788792795796798800804807818826831832833834840850854858860862876882886887894958959960962963964965966967968969970971972973974975976977978979980981982983984985986987988989990991992993994995996997998999") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"AF\0AL\0AQ\0DZ\0AS\0AD\0AO\0AG\0AZ\0AR\0AU\0AT\0BS\0BH\0BD\0AM\0BB\0BE\0BM\0BT\0BO\0BA\0BW\0BV\0BR\0BZ\0IO\0SB\0VG\0BN\0BG\0MM\0BI\0BY\0KH\0CM\0CA\0CV\0KY\0CF\0LK\0TD\0CL\0CN\0TW\0CX\0CC\0CO\0KM\0YT\0CG\0CD\0CK\0CR\0HR\0CU\0CY\0CZ\0BJ\0DK\0DM\0DO\0EC\0SV\0GQ\0ET\0ET\0ER\0EE\0FO\0FK\0GS\0FJ\0FI\0AX\0FR\0FR\0GF\0PF\0TF\0DJ\0GA\0GE\0GM\0PS\0DE\0DE\0DE\0GH\0GI\0KI\0GR\0GL\0GD\0GP\0GU\0GT\0GN\0GY\0HT\0HM\0VA\0HN\0HK\0HU\0IS\0IN\0ID\0IR\0IQ\0IE\0IL\0IT\0CI\0JM\0JP\0KZ\0JO\0KE\0KP\0KR\0KW\0KG\0LA\0LB\0LS\0LV\0LR\0LY\0LI\0LT\0LU\0MO\0MG\0MW\0MY\0MV\0ML\0MT\0MQ\0MR\0MU\0MX\0MC\0MN\0MD\0ME\0MS\0MA\0MZ\0OM\0NA\0NR\0NP\0NL\0CW\0AW\0SX\0BQ\0NC\0VU\0NZ\0NI\0NE\0NG\0NU\0NF\0NO\0MP\0UM\0FM\0MH\0PW\0PK\0PA\0PG\0PY\0PE\0PH\0PN\0PL\0PT\0GW\0TL\0PR\0QA\0RE\0RO\0RU\0RW\0BL\0SH\0KN\0AI\0LC\0MF\0PM\0VC\0SM\0ST\0SA\0SN\0RS\0SC\0SL\0SG\0SK\0VN\0SI\0SO\0ZA\0ZW\0YE\0ES\0SS\0SD\0EH\0SD\0SR\0SJ\0SZ\0SE\0CH\0SY\0TJ\0TH\0TG\0TK\0TO\0TT\0AE\0TN\0TR\0TM\0TC\0TV\0UG\0UA\0MK\0EG\0GB\0GG\0JE\0IM\0TZ\0US\0VI\0BF\0UY\0UZ\0VE\0WF\0WS\0YE\0YE\0ZM\0AA\0QM\0QN\0QP\0QQ\0QR\0QS\0QT\0EU\0QV\0QW\0QX\0QY\0QZ\0XA\0XB\0XC\0XD\0XE\0XF\0XG\0XH\0XI\0XJ\0XK\0XL\0XM\0XN\0XO\0XP\0XQ\0XR\0XS\0XT\0XU\0XV\0XW\0XX\0XY\0XZ\0ZZ\0") })
                },
                complex_region: unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"062172200530532536582810830890891AN\0CS\0FQ\0NT\0PC\0SU\0YU\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x12\0\0\0\0\0\x06\0*\x000\09\0B\0H\0T\0\x81\0\x87\0\x99\0\x9F\0\xA8\0\xAE\0\xB4\0\xBA\0\xC6\0\xF3\x00034143RU\0AM\0AZ\0BY\0GE\0KG\0KZ\0MD\0TJ\0TM\0UA\0UZ\0CZ\0SK\0CW\0SX\0BQ\0CW\0SX\0BQ\0SA\0IQ\0FM\0MH\0MP\0PW\0RU\0AM\0AZ\0BY\0EE\0GE\0KZ\0KG\0LV\0LT\0MD\0TJ\0TM\0UA\0UZ\0JE\0GG\0RS\0ME\0SI\0HR\0MK\0BA\0RS\0ME\0CW\0SX\0BQ\0RS\0ME\0AQ\0TF\0SA\0IQ\0FM\0MH\0MP\0PW\0RU\0AM\0AZ\0BY\0EE\0GE\0KZ\0KG\0LV\0LT\0MD\0TJ\0TM\0UA\0UZ\0RS\0ME\0") })
                },
                variant: unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"heploc\0\0polytoni") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"alalc97\0polyton\0") })
                },
                subdivision: unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"cn11\0\0\0cn12\0\0\0cn13\0\0\0cn14\0\0\0cn15\0\0\0cn21\0\0\0cn22\0\0\0cn23\0\0\0cn31\0\0\0cn32\0\0\0cn33\0\0\0cn34\0\0\0cn35\0\0\0cn36\0\0\0cn37\0\0\0cn41\0\0\0cn42\0\0\0cn43\0\0\0cn44\0\0\0cn45\0\0\0cn46\0\0\0cn50\0\0\0cn51\0\0\0cn52\0\0\0cn53\0\0\0cn54\0\0\0cn61\0\0\0cn62\0\0\0cn63\0\0\0cn64\0\0\0cn65\0\0\0cn71\0\0\0cn91\0\0\0cn92\0\0\0cz10a\0\0cz10b\0\0cz10c\0\0cz10d\0\0cz10e\0\0cz10f\0\0cz611\0\0cz612\0\0cz613\0\0cz614\0\0cz615\0\0cz621\0\0cz622\0\0cz623\0\0cz624\0\0cz626\0\0cz627\0\0czjc\0\0\0czjm\0\0\0czka\0\0\0czkr\0\0\0czli\0\0\0czmo\0\0\0czol\0\0\0czpa\0\0\0czpl\0\0\0czpr\0\0\0czst\0\0\0czus\0\0\0czvy\0\0\0czzl\0\0\0fi01\0\0\0fra\0\0\0\0frb\0\0\0\0frbl\0\0\0frc\0\0\0\0frcp\0\0\0frd\0\0\0\0fre\0\0\0\0frf\0\0\0\0frg\0\0\0\0frgf\0\0\0frgp\0\0\0frh\0\0\0\0fri\0\0\0\0frj\0\0\0\0frk\0\0\0\0frl\0\0\0\0frm\0\0\0\0frmf\0\0\0frmq\0\0\0frn\0\0\0\0frnc\0\0\0fro\0\0\0\0frp\0\0\0\0frpf\0\0\0frpm\0\0\0frq\0\0\0\0frr\0\0\0\0frre\0\0\0frs\0\0\0\0frt\0\0\0\0frtf\0\0\0fru\0\0\0\0frv\0\0\0\0frwf\0\0\0fryt\0\0\0laxn\0\0\0lud\0\0\0\0lug\0\0\0\0lul\0\0\0\0mrnkc\0\0nlaw\0\0\0nlcw\0\0\0nlsx\0\0\0no23\0\0\0nzn\0\0\0\0nzs\0\0\0\0omba\0\0\0omsh\0\0\0plds\0\0\0plkp\0\0\0pllb\0\0\0plld\0\0\0pllu\0\0\0plma\0\0\0plmz\0\0\0plop\0\0\0plpd\0\0\0plpk\0\0\0plpm\0\0\0plsk\0\0\0plsl\0\0\0plwn\0\0\0plwp\0\0\0plzp\0\0\0shta\0\0\0tteto\0\0ttrcm\0\0ttwto\0\0twkhq\0\0twtnq\0\0twtpq\0\0twtxq\0\0usas\0\0\0usgu\0\0\0usmp\0\0\0uspr\0\0\0usum\0\0\0usvi\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"cnbj\0\0\0cntj\0\0\0cnhe\0\0\0cnsx\0\0\0cnmn\0\0\0cnln\0\0\0cnjl\0\0\0cnhl\0\0\0cnsh\0\0\0cnjs\0\0\0cnzj\0\0\0cnah\0\0\0cnfj\0\0\0cnjx\0\0\0cnsd\0\0\0cnha\0\0\0cnhb\0\0\0cnhn\0\0\0cngd\0\0\0cngx\0\0\0cnhi\0\0\0cncq\0\0\0cnsc\0\0\0cngz\0\0\0cnyn\0\0\0cnxz\0\0\0cnsn\0\0\0cngs\0\0\0cnqh\0\0\0cnnx\0\0\0cnxj\0\0\0twzzzz\0hkzzzz\0mozzzz\0cz110\0\0cz111\0\0cz112\0\0cz113\0\0cz114\0\0cz115\0\0cz663\0\0cz632\0\0cz633\0\0cz634\0\0cz635\0\0cz641\0\0cz642\0\0cz643\0\0cz644\0\0cz646\0\0cz647\0\0cz31\0\0\0cz64\0\0\0cz41\0\0\0cz52\0\0\0cz51\0\0\0cz80\0\0\0cz71\0\0\0cz53\0\0\0cz32\0\0\0cz10\0\0\0cz20\0\0\0cz42\0\0\0cz63\0\0\0cz72\0\0\0axzzzz\0frges\0\0frnaq\0\0blzzzz\0frara\0\0cpzzzz\0frbfc\0\0frbre\0\0frcvl\0\0frges\0\0gfzzzz\0gpzzzz\0frcor\0\0frbfc\0\0fridf\0\0frocc\0\0frnaq\0\0frges\0\0mfzzzz\0mqzzzz\0frocc\0\0nczzzz\0frhdf\0\0frnor\0\0pfzzzz\0pmzzzz\0frnor\0\0frpdl\0\0rezzzz\0frhdf\0\0frnaq\0\0tfzzzz\0frpac\0\0frara\0\0wfzzzz\0ytzzzz\0laxs\0\0\0lucl\0\0\0luec\0\0\0luca\0\0\0mr13\0\0\0awzzzz\0cwzzzz\0sxzzzz\0no50\0\0\0nzauk\0\0nzcan\0\0ombj\0\0\0omsj\0\0\0pl02\0\0\0pl04\0\0\0pl08\0\0\0pl10\0\0\0pl06\0\0\0pl12\0\0\0pl14\0\0\0pl16\0\0\0pl20\0\0\0pl18\0\0\0pl22\0\0\0pl26\0\0\0pl24\0\0\0pl28\0\0\0pl30\0\0\0pl32\0\0\0tazzzz\0tttob\0\0ttmrc\0\0tttob\0\0twkhh\0\0twtnn\0\0twnwt\0\0twtxg\0\0aszzzz\0guzzzz\0mpzzzz\0przzzz\0umzzzz\0vizzzz\0") })
                },
            };
        }
        #[clippy::msrv = "1.67"]
        impl icu_provider::DataProvider<icu::locid_transform::provider::AliasesV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu::locid_transform::provider::AliasesV1Marker>, icu_provider::DataError> {
                if req.locale.is_empty() {
                    Ok(icu_provider::DataResponse { payload: Some(icu_provider::DataPayload::from_static_ref(Self::SINGLETON_LOCID_TRANSFORM_ALIASES_V1)), metadata: Default::default() })
                } else {
                    Err(icu_provider::DataErrorKind::ExtraneousLocale.with_req(<icu::locid_transform::provider::AliasesV1Marker as icu_provider::KeyedDataMarker>::KEY, req))
                }
            }
        }
    };
}
