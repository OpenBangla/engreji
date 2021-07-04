use engreji::generate_regex;
use regex::Regex;

#[test]
fn test_word_tech() {
    let regex = generate_regex("tech");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("টেক"));
}

#[test]
fn test_word_hack() {
    let regex = generate_regex("hack");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("হ্যাক"));
}

#[test]
fn test_word_finance() {
    let regex = generate_regex("finance");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ফাইনান্স"));
}

#[test]
fn test_word_guard() {
    let regex = generate_regex("guard");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("গার্ড"));
}

#[test]
fn test_word_empire() {
    let regex = generate_regex("empire");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("এম্পায়ার"));
}

#[test]
fn test_word_boolean() {
    let regex = generate_regex("boolean");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("বুলিয়ান"));
}

#[test]
fn test_word_journalist() {
    let regex = generate_regex("journalist");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("জার্নালিস্ট"));
}

#[test]
fn test_word_profile() {
    let regex = generate_regex("profile");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("প্রোফাইল"));
}

#[test]
fn test_word_table() {
    let regex = generate_regex("table");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("টেবিল"));
}

#[test]
fn test_word_device() {
    let regex = generate_regex("device");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ডিভাইস"));
}

#[test]
fn test_word_digital() {
    let regex = generate_regex("digital");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ডিজিটাল"));
}

#[test]
fn test_word_chemical() {
    let regex = generate_regex("chemical");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("কেমিক্যাল"));
}

#[test]
fn test_word_shopping() {
    let regex = generate_regex("shopping");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("শপিং"));
}

#[test]
fn test_word_game() {
    let regex = generate_regex("game");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("গেম"));
}

#[test]
fn test_word_xerox() {
    let regex = generate_regex("xerox");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("জেরক্স"));
}

#[test]
fn test_word_picture() {
    let regex = generate_regex("picture");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("পিকচার"));
}

#[test]
fn test_word_preview() {
    let regex = generate_regex("preview");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("প্রিভিউ"));
}

#[test]
fn test_word_portable() {
    let regex = generate_regex("portable");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("পোর্টেবল"));
}

#[test]
fn test_word_home() {
    let regex = generate_regex("home");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("হোম"));
}

#[test]
fn test_word_chairman() {
    let regex = generate_regex("chairman");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("চেয়ারম্যান"));
}

#[test]
fn test_word_football() {
    let regex = generate_regex("football");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ফুটবল"));
}

#[test]
fn test_word_cell() {
    let regex = generate_regex("cell");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("সেল"));
}

#[test]
fn test_word_spy() {
    let regex = generate_regex("spy");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("স্পাই"));
}

#[test]
fn test_word_messenger() {
    let regex = generate_regex("messenger");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("মেসেঞ্জার"));
}

#[test]
fn test_word_microphone() {
    let regex = generate_regex("microphone");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("মাইক্রোফোন"));
}

#[test]
fn test_word_file() {
    let regex = generate_regex("file");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ফাইল"));
}

#[test]
fn test_word_maximize() {
    let regex = generate_regex("maximize");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ম্যাক্সিমাইজ"));
}

#[test]
fn test_word_publisher() {
    let regex = generate_regex("publisher");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("পাবলিশার"));
}

#[test]
fn test_word_video() {
    let regex = generate_regex("video");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ভিডিও"));
}

#[test]
fn test_word_entries() {
    let regex = generate_regex("entries");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("এন্ট্রিজ"));
}

#[test]
fn test_word_icon() {
    let regex = generate_regex("icon");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("আইকন"));
}

#[test]
fn test_word_panel() {
    let regex = generate_regex("panel");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("প্যানেল"));
}

#[test]
fn test_word_code() {
    let regex = generate_regex("code");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("কোড"));
}

#[test]
fn test_word_application() {
    let regex = generate_regex("application");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("অ্যাপ্লিকেশন"));
}

#[test]
fn test_word_lotion() {
    let regex = generate_regex("lotion");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("লোশন"));
}

#[test]
fn test_word_typing() {
    let regex = generate_regex("typing");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("টাইপিং"));
}

#[test]
fn test_word_net() {
    let regex = generate_regex("net");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("নেট"));
}

#[test]
fn test_word_textile() {
    let regex = generate_regex("textile");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("টেক্সটাইল"));
}

#[test]
fn test_word_windows() {
    let regex = generate_regex("windows");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("উইন্ডোজ"));
}

#[test]
fn test_word_racing() {
    let regex = generate_regex("racing");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("রেসিং"));
}

#[test]
fn test_word_design() {
    let regex = generate_regex("design");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ডিজাইন"));
}

#[test]
fn test_word_windscreen() {
    let regex = generate_regex("windscreen");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("উইন্ডস্ক্রিন"));
}

#[test]
fn test_word_global() {
    let regex = generate_regex("global");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("গ্লোবাল"));
}

#[test]
fn test_word_international() {
    let regex = generate_regex("international");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ইন্টারন্যাশনাল"));
}

#[test]
fn test_word_africa() {
    let regex = generate_regex("africa");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("আফ্রিকা"));
}

#[test]
fn test_word_fedora() {
    let regex = generate_regex("fedora");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ফেডোরা"));
}

#[test]
fn test_word_institute() {
    let regex = generate_regex("institute");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ইন্সিটিউট"));
}

#[test]
fn test_word_mobile() {
    let regex = generate_regex("mobile");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("মোবাইল"));
}

#[test]
fn test_word_olympic() {
    let regex = generate_regex("olympic");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("অলিম্পিক"));
}

#[test]
fn test_word_auditorium() {
    let regex = generate_regex("auditorium");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("অডিটোরিয়াম"));
}

#[test]
fn test_word_street() {
    let regex = generate_regex("street");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("স্ট্রিট"));
}

#[test]
fn test_word_shoe() {
    let regex = generate_regex("shoe");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("শু"));
}

#[test]
fn test_word_view() {
    let regex = generate_regex("view");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ভিউ"));
}

#[test]
fn test_word_biology() {
    let regex = generate_regex("biology");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("বায়োলজি"));
}

#[test]
fn test_word_rack() {
    let regex = generate_regex("rack");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("র‍্যাক"));
}

#[test]
fn test_word_technology() {
    let regex = generate_regex("technology");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("টেকনোলজি"));
}

#[test]
fn test_word_states() {
    let regex = generate_regex("states");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("স্টেটস"));
}

#[test]
fn test_word_great() {
    let regex = generate_regex("great");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("গ্রেট"));
}

#[test]
fn test_word_motherboard() {
    let regex = generate_regex("motherboard");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("মাদারবোর্ড"));
}

#[test]
fn test_word_remove() {
    let regex = generate_regex("remove");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("রিমুভ"));
}

#[test]
fn test_word_police() {
    let regex = generate_regex("police");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("পুলিশ"));
}

#[test]
fn test_word_taxi() {
    let regex = generate_regex("taxi");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ট্যাক্সি"));
}

#[test]
fn test_word_motor() {
    let regex = generate_regex("motor");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("মোটর"));
}

#[test]
fn test_word_torrent() {
    let regex = generate_regex("torrent");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("টরেন্ট"));
}

#[test]
fn test_word_window() {
    let regex = generate_regex("window");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("উইন্ডো"));
}

#[test]
fn test_word_dashboard() {
    let regex = generate_regex("dashboard");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ড্যাশবোর্ড"));
}

#[test]
fn test_word_stupid() {
    let regex = generate_regex("stupid");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("স্টুপিড"));
}

#[test]
fn test_word_virtual() {
    let regex = generate_regex("virtual");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ভার্চুয়াল"));
}

#[test]
fn test_word_paint() {
    let regex = generate_regex("paint");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("পেইন্ট"));
}

#[test]
fn test_word_action() {
    let regex = generate_regex("action");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("অ্যাকশন"));
}

#[test]
fn test_word_series() {
    let regex = generate_regex("series");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("সিরিজ"));
}

#[test]
fn test_word_college() {
    let regex = generate_regex("college");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("কলেজ"));
}

#[test]
fn test_word_text() {
    let regex = generate_regex("text");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("টেক্সট"));
}

#[test]
fn test_word_skirt() {
    let regex = generate_regex("skirt");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("স্কার্ট"));
}

#[test]
fn test_word_panty() {
    let regex = generate_regex("panty");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("প্যান্টি"));
}

#[test]
fn test_word_player() {
    let regex = generate_regex("player");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("প্লেয়ার"));
}

#[test]
fn test_word_courier() {
    let regex = generate_regex("courier");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ক্যুরিয়ার"));
}

#[test]
fn test_word_napkin() {
    let regex = generate_regex("napkin");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ন্যাপকিন"));
}

#[test]
fn test_word_designer() {
    let regex = generate_regex("designer");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ডিজাইনার"));
}

#[test]
fn test_word_autopsy() {
    let regex = generate_regex("autopsy");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("অটোপ্সি"));
}

#[test]
fn test_word_projector() {
    let regex = generate_regex("projector");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("প্রোজেক্টর"));
}

#[test]
fn test_word_advertisement() {
    let regex = generate_regex("advertisement");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("অ্যাডভার্টাইজমেন্ট"));
}

#[test]
fn test_word_cargo() {
    let regex = generate_regex("cargo");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("কার্গো"));
}

#[test]
fn test_word_shocker() {
    let regex = generate_regex("shocker");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("শকার"));
}

#[test]
fn test_word_word() {
    let regex = generate_regex("word");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ওয়ার্ড"));
}

#[test]
fn test_word_reception() {
    let regex = generate_regex("reception");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("রিসেপশন"));
}

#[test]
fn test_word_search() {
    let regex = generate_regex("search");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("সার্চ"));
}

#[test]
fn test_word_option() {
    let regex = generate_regex("option");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("অপশন"));
}

#[test]
fn test_word_under() {
    let regex = generate_regex("under");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("আন্ডার"));
}

#[test]
fn test_word_numeric() {
    let regex = generate_regex("numeric");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("নিউমেরিক"));
}

#[test]
fn test_word_manager() {
    let regex = generate_regex("manager");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ম্যানেজার"));
}


#[test]
fn test_word_professor() {
    let regex = generate_regex("professor");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("প্রোফেসর"));
}

#[test]
fn test_word_power() {
    let regex = generate_regex("power");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("পাওয়ার"));
}

#[test]
fn test_word_jet() {
    let regex = generate_regex("jet");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("জেট"));
}

#[test]
fn test_word_conference() {
    let regex = generate_regex("conference");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("কনফারেন্স"));
}

#[test]
fn test_word_credit() {
    let regex = generate_regex("credit");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ক্রেডিট"));
}

#[test]
fn test_word_court() {
    let regex = generate_regex("court");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("কোর্ট"));
}

#[test]
fn test_word_drawing() {
    let regex = generate_regex("drawing");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ড্রয়িং"));
}

#[test]
fn test_word_government() {
    let regex = generate_regex("government");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("গভর্নমেন্ট"));
}

#[test]
fn test_word_download() {
    let regex = generate_regex("download");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ডাউনলোড"));
}

#[test]
fn test_word_arabia() {
    let regex = generate_regex("arabia");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("আরব"));
}

#[test]
fn test_word_optical() {
    let regex = generate_regex("optical");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("অপ্টিকাল"));
}

#[test]
fn test_word_light() {
    let regex = generate_regex("light");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("লাইট"));
}

#[test]
fn test_word_technician() {
    let regex = generate_regex("technician");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("টেকনিশিয়ান"));
}

#[test]
fn test_word_march() {
    let regex = generate_regex("march");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("মার্চ"));
}

#[test]
fn test_word_clip() {
    let regex = generate_regex("clip");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ক্লিপ"));
}

#[test]
fn test_word_italy() {
    let regex = generate_regex("italy");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ইটালি"));
}

#[test]
fn test_word_night() {
    let regex = generate_regex("night");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("নাইট"));
}

#[test]
fn test_word_year() {
    let regex = generate_regex("year");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ইয়ার"));
}

#[test]
fn test_word_folder() {
    let regex = generate_regex("folder");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ফোল্ডার"));
}

#[test]
fn test_word_siemens() {
    let regex = generate_regex("siemens");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("সিমেন্স"));
}

#[test]
fn test_word_money() {
    let regex = generate_regex("money");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("মানি"));
}

#[test]
fn test_word_pack() {
    let regex = generate_regex("pack");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("প্যাক"));
}

#[test]
fn test_word_replace() {
    let regex = generate_regex("replace");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("রিপ্লেস"));
}

#[test]
fn test_word_railway() {
    let regex = generate_regex("railway");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("রেলওয়ে"));
}

#[test]
fn test_word_filter() {
    let regex = generate_regex("filter");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ফিল্টার"));
}

#[test]
fn test_word_coin() {
    let regex = generate_regex("coin");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("কয়েন"));
}

#[test]
fn test_word_ragging() {
    let regex = generate_regex("ragging");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("র‍্যাগিং"));
}

#[test]
fn test_word_kilo() {
    let regex = generate_regex("kilo");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("কিলো"));
}

#[test]
fn test_word_canada() {
    let regex = generate_regex("canada");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("কানাডা"));
}

#[test]
fn test_word_operating() {
    let regex = generate_regex("operating");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("অপারেটিং"));
}

#[test]
fn test_word_postpaid() {
    let regex = generate_regex("postpaid");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("পোস্টপেড"));
}

#[test]
fn test_word_machine() {
    let regex = generate_regex("machine");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("মেশিন"));
}

#[test]
fn test_word_potter() {
    let regex = generate_regex("potter");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("পটার"));
}

#[test]
fn test_word_truck() {
    let regex = generate_regex("truck");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ট্রাক"));
}

#[test]
fn test_word_brush() {
    let regex = generate_regex("brush");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ব্রাশ"));
}

#[test]
fn test_word_correct() {
    let regex = generate_regex("correct");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("কারেক্ট"));
}

#[test]
fn test_word_warning() {
    let regex = generate_regex("warning");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ওয়ার্নিং"));
}

#[test]
fn test_word_hardware() {
    let regex = generate_regex("hardware");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("হার্ডওয়্যার"));
}

#[test]
fn test_word_flash() {
    let regex = generate_regex("flash");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ফ্ল্যাশ"));
}

#[test]
fn test_word_pentagon() {
    let regex = generate_regex("pentagon");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("পেন্টাগন"));
}

#[test]
fn test_word_hockey() {
    let regex = generate_regex("hockey");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("হকি"));
}

#[test]
fn test_word_layer() {
    let regex = generate_regex("layer");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("লেয়ার"));
}

#[test]
fn test_word_syria() {
    let regex = generate_regex("syria");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("সিরিয়া"));
}

#[test]
fn test_word_mount() {
    let regex = generate_regex("mount");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("মাউন্ট"));
}

#[test]
fn test_word_artist() {
    let regex = generate_regex("artist");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("আর্টিস্ট"));
}

#[test]
fn test_word_minister() {
    let regex = generate_regex("minister");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("মিনিস্টার"));
}

#[test]
fn test_word_recipe() {
    let regex = generate_regex("recipe");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("রেসিপি"));
}

#[test]
fn test_word_australia() {
    let regex = generate_regex("australia");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("অস্ট্রেলিয়া"));
}

#[test]
fn test_word_cancel() {
    let regex = generate_regex("cancel");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ক্যানসেল"));
}

#[test]
fn test_word_edit() {
    let regex = generate_regex("edit");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("এডিট"));
}

#[test]
fn test_word_asia() {
    let regex = generate_regex("asia");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("এশিয়া"));
}

#[test]
fn test_word_default() {
    let regex = generate_regex("default");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ডিফল্ট"));
}

#[test]
fn test_word_backspace() {
    let regex = generate_regex("backspace");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ব্যাকস্পেস"));
}

#[test]
fn test_word_phone() {
    let regex = generate_regex("phone");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ফোন"));
}

#[test]
fn test_word_capital() {
    let regex = generate_regex("capital");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ক্যাপিটাল"));
}

#[test]
fn test_word_complain() {
    let regex = generate_regex("complain");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("কমপ্লেইন"));
}

#[test]
fn test_word_bank() {
    let regex = generate_regex("bank");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ব্যাংক"));
}

#[test]
fn test_word_version() {
    let regex = generate_regex("version");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ভার্সন"));
}

#[test]
fn test_word_cartoon() {
    let regex = generate_regex("cartoon");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("কার্টুন"));
}

#[test]
fn test_word_degree() {
    let regex = generate_regex("degree");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ডিগ্রি"));
}

#[test]
fn test_word_racket() {
    let regex = generate_regex("racket");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("র‍্যাকেট"));
}

#[test]
fn test_word_lecture() {
    let regex = generate_regex("lecture");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("লেকচার"));
}

#[test]
fn test_word_convert() {
    let regex = generate_regex("convert");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("কনভার্ট"));
}

#[test]
fn test_word_developer() {
    let regex = generate_regex("developer");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ডেভেলপার"));
}

#[test]
fn test_word_crime() {
    let regex = generate_regex("crime");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ক্রাইম"));
}

#[test]
fn test_word_forensic() {
    let regex = generate_regex("forensic");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ফরেন্সিক"));
}

#[test]
fn test_word_europe() {
    let regex = generate_regex("europe");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ইউরোপ"));
}

#[test]
fn test_word_exchange() {
    let regex = generate_regex("exchange");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("এক্সচেঞ্জ"));
}

#[test]
fn test_word_floppy() {
    let regex = generate_regex("floppy");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ফ্লপি"));
}

#[test]
fn test_word_fossil() {
    let regex = generate_regex("fossil");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ফসিল"));
}

#[test]
fn test_word_notes() {
    let regex = generate_regex("notes");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("নোটস"));
}

#[test]
fn test_word_post() {
    let regex = generate_regex("post");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("পোস্ট"));
}

#[test]
fn test_word_girl() {
    let regex = generate_regex("girl");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("গার্ল"));
}

#[test]
fn test_word_custom() {
    let regex = generate_regex("custom");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("কাস্টম"));
}

#[test]
fn test_word_matador() {
    let regex = generate_regex("matador");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ম্যাটাডর"));
}

#[test]
fn test_word_history() {
    let regex = generate_regex("history");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("হিস্ট্রি"));
}

#[test]
fn test_word_tomato() {
    let regex = generate_regex("tomato");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("টমেটো"));
}

#[test]
fn test_word_heart() {
    let regex = generate_regex("heart");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("হার্ট"));
}

#[test]
fn test_word_bit() {
    let regex = generate_regex("bit");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("বিট"));
}

#[test]
fn test_word_generation() {
    let regex = generate_regex("generation");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("জেনারেশন"));
}

#[test]
fn test_word_life() {
    let regex = generate_regex("life");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("লাইফ"));
}

#[test]
fn test_word_place() {
    let regex = generate_regex("place");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("প্লেস"));
}

#[test]
fn test_word_lab() {
    let regex = generate_regex("lab");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ল্যাব"));
}

#[test]
fn test_word_muffler() {
    let regex = generate_regex("muffler");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("মাফলার"));
}

#[test]
fn test_word_size() {
    let regex = generate_regex("size");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("সাইজ"));
}

#[test]
fn test_word_production() {
    let regex = generate_regex("production");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("প্রোডাকশন"));
}

#[test]
fn test_word_total() {
    let regex = generate_regex("total");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("টোটাল"));
}

#[test]
fn test_word_writer() {
    let regex = generate_regex("writer");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("রাইটার"));
}

#[test]
fn test_word_presentation() {
    let regex = generate_regex("presentation");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("প্রেজেন্টেশন"));
}

#[test]
fn test_word_greece() {
    let regex = generate_regex("greece");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("গ্রিস"));
}

#[test]
fn test_word_pregnancy() {
    let regex = generate_regex("pregnancy");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("প্রেগন্যান্সি"));
}

#[test]
fn test_word_fine() {
    let regex = generate_regex("fine");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ফাইন"));
}

#[test]
fn test_word_letterhead() {
    let regex = generate_regex("letterhead");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("লেটারহেড"));
}

#[test]
fn test_word_outbox() {
    let regex = generate_regex("outbox");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("আউটবক্স"));
}

#[test]
fn test_word_printer() {
    let regex = generate_regex("printer");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("প্রিন্টার"));
}

#[test]
fn test_word_radio() {
    let regex = generate_regex("radio");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("রেডিও"));
}

#[test]
fn test_word_bicycle() {
    let regex = generate_regex("bicycle");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("বাইসাইকেল"));
}

#[test]
fn test_word_fantasy() {
    let regex = generate_regex("fantasy");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ফ্যান্টাসি"));
}

#[test]
fn test_word_lock() {
    let regex = generate_regex("lock");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("লক"));
}

#[test]
fn test_word_product() {
    let regex = generate_regex("product");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("প্রোডাক্ট"));
}

#[test]
fn test_word_select() {
    let regex = generate_regex("select");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("সিলেক্ট"));
}

#[test]
fn test_word_processor() {
    let regex = generate_regex("processor");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("প্রসেসর"));
}

#[test]
fn test_word_form() {
    let regex = generate_regex("form");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ফর্ম"));
}

#[test]
fn test_word_circus() {
    let regex = generate_regex("circus");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("সার্কাস"));
}

#[test]
fn test_word_switch() {
    let regex = generate_regex("switch");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("সুইচ"));
}

#[test]
fn test_word_adjust() {
    let regex = generate_regex("adjust");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("অ্যাডজাস্ট"));
}

#[test]
fn test_word_bulb() {
    let regex = generate_regex("bulb");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("বাল্ব"));
}

#[test]
fn test_word_richard() {
    let regex = generate_regex("richard");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("রিচার্ড"));
}

#[test]
fn test_word_excel() {
    let regex = generate_regex("excel");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("এক্সেল"));
}

#[test]
fn test_word_add() {
    let regex = generate_regex("add");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("অ্যাড"));
}

#[test]
fn test_word_input() {
    let regex = generate_regex("input");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ইনপুট"));
}

#[test]
fn test_word_interface() {
    let regex = generate_regex("interface");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ইন্টারফেস"));
}

#[test]
fn test_word_encoding() {
    let regex = generate_regex("encoding");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("এনকোডিং"));
}

#[test]
fn test_word_foundation() {
    let regex = generate_regex("foundation");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ফাউন্ডেশন"));
}

#[test]
fn test_word_hotel() {
    let regex = generate_regex("hotel");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("হোটেল"));
}

#[test]
fn test_word_star() {
    let regex = generate_regex("star");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("স্টার"));
}

#[test]
fn test_word_trademark() {
    let regex = generate_regex("trademark");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ট্রেডমার্ক"));
}

#[test]
fn test_word_wow() {
    let regex = generate_regex("wow");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ওয়াও"));
}

#[test]
fn test_word_south() {
    let regex = generate_regex("south");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("সাউথ"));
}

#[test]
fn test_word_notepad() {
    let regex = generate_regex("notepad");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("নোটপ্যাড"));
}

#[test]
fn test_word_prime() {
    let regex = generate_regex("prime");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("প্রাইম"));
}

#[test]
fn test_word_soft() {
    let regex = generate_regex("soft");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("সফট"));
}

#[test]
fn test_word_hype() {
    let regex = generate_regex("hype");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("হাইপ"));
}

#[test]
fn test_word_converter() {
    let regex = generate_regex("converter");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("কনভার্টার"));
}

#[test]
fn test_word_stick() {
    let regex = generate_regex("stick");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("স্টিক"));
}

#[test]
fn test_word_lace() {
    let regex = generate_regex("lace");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("লেস"));
}

#[test]
fn test_word_program() {
    let regex = generate_regex("program");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("প্রোগ্রাম"));
}

#[test]
fn test_word_germany() {
    let regex = generate_regex("germany");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("জার্মানি"));
}

#[test]
fn test_word_dutch() {
    let regex = generate_regex("dutch");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ডাচ"));
}

#[test]
fn test_word_footage() {
    let regex = generate_regex("footage");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ফুটেজ"));
}

#[test]
fn test_word_west() {
    let regex = generate_regex("west");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ওয়েস্ট"));
}

#[test]
fn test_word_internet() {
    let regex = generate_regex("internet");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ইন্টারনেট"));
}

#[test]
fn test_word_dollar() {
    let regex = generate_regex("dollar");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ডলার"));
}

#[test]
fn test_word_definitely() {
    let regex = generate_regex("definitely");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ডেফিনেটলি"));
}

#[test]
fn test_word_scrap() {
    let regex = generate_regex("scrap");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("স্ক্র্যাপ"));
}

#[test]
fn test_word_support() {
    let regex = generate_regex("support");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("সাপোর্ট"));
}

#[test]
fn test_word_play() {
    let regex = generate_regex("play");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("প্লে"));
}

#[test]
fn test_word_media() {
    let regex = generate_regex("media");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("মিডিয়া"));
}

#[test]
fn test_word_postmortem() {
    let regex = generate_regex("postmortem");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("পোস্টমর্টেম"));
}

#[test]
fn test_word_traffic() {
    let regex = generate_regex("traffic");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ট্রাফিক"));
}

#[test]
fn test_word_task() {
    let regex = generate_regex("task");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("টাস্ক"));
}

#[test]
fn test_word_right() {
    let regex = generate_regex("right");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("রাইট"));
}

#[test]
fn test_word_pharmacy() {
    let regex = generate_regex("pharmacy");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ফার্মেসি"));
}

#[test]
fn test_word_chrome() {
    let regex = generate_regex("chrome");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ক্রোম"));
}

#[test]
fn test_word_charger() {
    let regex = generate_regex("charger");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("চার্জার"));
}

#[test]
fn test_word_race() {
    let regex = generate_regex("race");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("রেস"));
}

#[test]
fn test_word_indian() {
    let regex = generate_regex("indian");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ইন্ডিয়ান"));
}

#[test]
fn test_word_print() {
    let regex = generate_regex("print");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("প্রিন্ট"));
}

#[test]
fn test_word_back() {
    let regex = generate_regex("back");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ব্যাক"));
}

#[test]
fn test_word_party() {
    let regex = generate_regex("party");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("পার্টি"));
}

#[test]
fn test_word_square() {
    let regex = generate_regex("square");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("স্কোয়ার"));
}

#[test]
fn test_word_canon() {
    let regex = generate_regex("canon");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ক্যানন"));
}

#[test]
fn test_word_head() {
    let regex = generate_regex("head");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("হেড"));
}

#[test]
fn test_word_find() {
    let regex = generate_regex("find");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ফাইন্ড"));
}

#[test]
fn test_word_resolution() {
    let regex = generate_regex("resolution");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("রেজোলিউশন"));
}

#[test]
fn test_word_engineering() {
    let regex = generate_regex("engineering");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ইঞ্জিনিয়ারিং"));
}

#[test]
fn test_word_language() {
    let regex = generate_regex("language");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ল্যাঙ্গুয়েজ"));
}

#[test]
fn test_word_tractor() {
    let regex = generate_regex("tractor");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ট্র্যাক্টর"));
}

#[test]
fn test_word_maximum() {
    let regex = generate_regex("maximum");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ম্যাক্সিমাম"));
}

#[test]
fn test_word_close() {
    let regex = generate_regex("close");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ক্লোজ"));
}

#[test]
fn test_word_tarzan() {
    let regex = generate_regex("tarzan");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("টারজান"));
}

#[test]
fn test_word_petro() {
    let regex = generate_regex("petro");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("পেট্রো"));
}

#[test]
fn test_word_guide() {
    let regex = generate_regex("guide");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("গাইড"));
}

#[test]
fn test_word_mechanism() {
    let regex = generate_regex("mechanism");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("মেকানিজম"));
}

#[test]
fn test_word_france() {
    let regex = generate_regex("france");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ফ্রান্স"));
}

#[test]
fn test_word_share() {
    let regex = generate_regex("share");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("শেয়ার"));
}

#[test]
fn test_word_fossils() {
    let regex = generate_regex("fossils");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ফসিলস"));
}

#[test]
fn test_word_entrance() {
    let regex = generate_regex("entrance");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("এন্ট্রান্স"));
}

#[test]
fn test_word_politics() {
    let regex = generate_regex("politics");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("পলিটিক্স"));
}

#[test]
fn test_word_steam() {
    let regex = generate_regex("steam");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("স্টিম"));
}

#[test]
fn test_word_enter() {
    let regex = generate_regex("enter");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("এন্টার"));
}

#[test]
fn test_word_properties() {
    let regex = generate_regex("properties");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("প্রপার্টিজ"));
}

#[test]
fn test_word_tokyo() {
    let regex = generate_regex("tokyo");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("টোকিয়ো"));
}

#[test]
fn test_word_key() {
    let regex = generate_regex("key");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("কি"));
}

#[test]
fn test_word_begum() {
    let regex = generate_regex("begum");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("বেগম"));
}

#[test]
fn test_word_editor() {
    let regex = generate_regex("editor");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("এডিটর"));
}

#[test]
fn test_word_site() {
    let regex = generate_regex("site");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("সাইট"));
}

#[test]
fn test_word_system() {
    let regex = generate_regex("system");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("সিস্টেম"));
}

#[test]
fn test_word_canvas() {
    let regex = generate_regex("canvas");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ক্যানভাস"));
}

#[test]
fn test_word_commission() {
    let regex = generate_regex("commission");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("কমিশন"));
}

#[test]
fn test_word_morgue() {
    let regex = generate_regex("morgue");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("মর্গ"));
}

#[test]
fn test_word_consortium() {
    let regex = generate_regex("consortium");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("কন্সর্টিয়াম"));
}

#[test]
fn test_word_reliance() {
    let regex = generate_regex("reliance");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("রিলায়েন্স"));
}

#[test]
fn test_word_resize() {
    let regex = generate_regex("resize");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("রিসাইজ"));
}

#[test]
fn test_word_american() {
    let regex = generate_regex("american");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("আমেরিকান"));
}

#[test]
fn test_word_flyover() {
    let regex = generate_regex("flyover");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ফ্লাইওভার"));
}

#[test]
fn test_word_negro() {
    let regex = generate_regex("negro");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("নিগ্রো"));
}

#[test]
fn test_word_publish() {
    let regex = generate_regex("publish");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("পাবলিশ"));
}

#[test]
fn test_word_conductor() {
    let regex = generate_regex("conductor");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("কন্ডাক্টর"));
}

#[test]
fn test_word_aeroplane() {
    let regex = generate_regex("aeroplane");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("এরোপ্লেন"));
}

#[test]
fn test_word_ie() {
    let regex = generate_regex("ie");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("আইই"));
}

#[test]
fn test_word_drive() {
    let regex = generate_regex("drive");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ড্রাইভ"));
}

#[test]
fn test_word_perfume() {
    let regex = generate_regex("perfume");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("পারফিউম"));
}

#[test]
fn test_word_bill() {
    let regex = generate_regex("bill");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("বিল"));
}

#[test]
fn test_word_macro() {
    let regex = generate_regex("macro");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ম্যাক্রো"));
}

#[test]
fn test_word_courtship() {
    let regex = generate_regex("courtship");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("কোর্টশিপ"));
}

#[test]
fn test_word_abdomen() {
    let regex = generate_regex("abdomen");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("অ্যাবডোমেন"));
}

#[test]
fn test_word_front() {
    let regex = generate_regex("front");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ফ্রন্ট"));
}

#[test]
fn test_word_managing() {
    let regex = generate_regex("managing");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ম্যানেজিং"));
}

#[test]
fn test_word_buzz() {
    let regex = generate_regex("buzz");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("বাজ"));
}

#[test]
fn test_word_letter() {
    let regex = generate_regex("letter");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("লেটার"));
}

#[test]
fn test_word_pause() {
    let regex = generate_regex("pause");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("পজ"));
}

#[test]
fn test_word_services() {
    let regex = generate_regex("services");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("সার্ভিসেস"));
}

#[test]
fn test_word_scotch() {
    let regex = generate_regex("scotch");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("স্কচ"));
}

#[test]
fn test_word_cycle() {
    let regex = generate_regex("cycle");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("সাইকেল"));
}

#[test]
fn test_word_picnic() {
    let regex = generate_regex("picnic");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("পিকনিক"));
}

#[test]
fn test_word_management() {
    let regex = generate_regex("management");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ম্যানেজমেন্ট"));
}

#[test]
fn test_word_certification() {
    let regex = generate_regex("certification");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("সার্টিফিকেশন"));
}

#[test]
fn test_word_office() {
    let regex = generate_regex("office");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("অফিস"));
}

#[test]
fn test_word_cable() {
    let regex = generate_regex("cable");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ক্যাবল"));
}

#[test]
fn test_word_sydney() {
    let regex = generate_regex("sydney");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("সিডনি"));
}

#[test]
fn test_word_programme() {
    let regex = generate_regex("programme");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("প্রোগ্রাম"));
}

#[test]
fn test_word_chinese() {
    let regex = generate_regex("chinese");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("চাইনীজ"));
}

#[test]
fn test_word_gland() {
    let regex = generate_regex("gland");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("গ্ল্যান্ড"));
}

#[test]
fn test_word_mongolia() {
    let regex = generate_regex("mongolia");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("মঙ্গোলিয়া"));
}

#[test]
fn test_word_oxford() {
    let regex = generate_regex("oxford");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("অক্সফোর্ড"));
}

#[test]
fn test_word_exercise() {
    let regex = generate_regex("exercise");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("এক্সারসাইজ"));
}

#[test]
fn test_word_next() {
    let regex = generate_regex("next");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("নেক্সট"));
}

#[test]
fn test_word_petrol() {
    let regex = generate_regex("petrol");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("পেট্রোল"));
}

#[test]
fn test_word_health() {
    let regex = generate_regex("health");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("হেলথ"));
}

#[test]
fn test_word_august() {
    let regex = generate_regex("august");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("অগাস্ট"));
}

#[test]
fn test_word_library() {
    let regex = generate_regex("library");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("লাইব্রেরি"));
}

#[test]
fn test_word_crest() {
    let regex = generate_regex("crest");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ক্রেস্ট"));
}

#[test]
fn test_word_tennis() {
    let regex = generate_regex("tennis");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("টেনিস"));
}

#[test]
fn test_word_cinema() {
    let regex = generate_regex("cinema");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("সিনেমা"));
}

#[test]
fn test_word_audio() {
    let regex = generate_regex("audio");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("অডিও"));
}

#[test]
fn test_word_progress() {
    let regex = generate_regex("progress");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("প্রোগ্রেস"));
}

#[test]
fn test_word_backup() {
    let regex = generate_regex("backup");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ব্যাকআপ"));
}

#[test]
fn test_word_service() {
    let regex = generate_regex("service");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("সার্ভিস"));
}

#[test]
fn test_word_mother() {
    let regex = generate_regex("mother");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("মাদার"));
}

#[test]
fn test_word_install() {
    let regex = generate_regex("install");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ইন্সটল"));
}

#[test]
fn test_word_toyota() {
    let regex = generate_regex("toyota");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("টয়োটা"));
}

#[test]
fn test_word_blouse() {
    let regex = generate_regex("blouse");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ব্লাউজ"));
}

#[test]
fn test_word_save() {
    let regex = generate_regex("save");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("সেভ"));
}

#[test]
fn test_word_injection() {
    let regex = generate_regex("injection");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ইঞ্জেকশন"));
}

#[test]
fn test_word_bloc() {
    let regex = generate_regex("bloc");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ব্লক"));
}

#[test]
fn test_word_trouser() {
    let regex = generate_regex("trouser");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ট্রাউজার"));
}

#[test]
fn test_word_infrared() {
    let regex = generate_regex("infrared");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ইনফ্রারেড"));
}

#[test]
fn test_word_north() {
    let regex = generate_regex("north");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("নর্থ"));
}

#[test]
fn test_word_launch() {
    let regex = generate_regex("launch");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("লঞ্চ"));
}

#[test]
fn test_word_shortcut() {
    let regex = generate_regex("shortcut");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("শর্টকাট"));
}

#[test]
fn test_word_debug() {
    let regex = generate_regex("debug");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ডিবাগ"));
}

#[test]
fn test_word_bullet() {
    let regex = generate_regex("bullet");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("বুলেট"));
}

#[test]
fn test_word_hostel() {
    let regex = generate_regex("hostel");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("হোস্টেল"));
}

#[test]
fn test_word_circle() {
    let regex = generate_regex("circle");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("সার্কেল"));
}

#[test]
fn test_word_digit() {
    let regex = generate_regex("digit");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ডিজিট"));
}

#[test]
fn test_word_mac() {
    let regex = generate_regex("mac");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ম্যাক"));
}

#[test]
fn test_word_mouse() {
    let regex = generate_regex("mouse");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("মাউস"));
}

#[test]
fn test_word_fixed() {
    let regex = generate_regex("fixed");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ফিক্সড"));
}

#[test]
fn test_word_document() {
    let regex = generate_regex("document");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ডকুমেন্ট"));
}

#[test]
fn test_word_button() {
    let regex = generate_regex("button");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("বাটন"));
}

#[test]
fn test_word_dinner() {
    let regex = generate_regex("dinner");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ডিনার"));
}

#[test]
fn test_word_criminal() {
    let regex = generate_regex("criminal");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ক্রিমিনাল"));
}

#[test]
fn test_word_medical() {
    let regex = generate_regex("medical");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("মেডিকেল"));
}

#[test]
fn test_word_active() {
    let regex = generate_regex("active");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("অ্যাক্টিভ"));
}

#[test]
fn test_word_palki() {
    let regex = generate_regex("palki");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("পালকি"));
}

#[test]
fn test_word_referee() {
    let regex = generate_regex("referee");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("রেফারি"));
}

#[test]
fn test_word_russia() {
    let regex = generate_regex("russia");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("রাশিয়া"));
}

#[test]
fn test_word_railways() {
    let regex = generate_regex("railways");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("রেলওয়েজ"));
}

#[test]
fn test_word_cake() {
    let regex = generate_regex("cake");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("কেক"));
}

#[test]
fn test_word_arab() {
    let regex = generate_regex("arab");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("আরব"));
}

#[test]
fn test_word_apple() {
    let regex = generate_regex("apple");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("অ্যাপল"));
}

#[test]
fn test_word_america() {
    let regex = generate_regex("america");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("আমেরিকা"));
}

#[test]
fn test_word_airway() {
    let regex = generate_regex("airway");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("এয়ারওয়ে"));
}

#[test]
fn test_word_doctor() {
    let regex = generate_regex("doctor");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ডক্টর"));
}

#[test]
fn test_word_multimedia() {
    let regex = generate_regex("multimedia");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("মাল্টিমিডিয়া"));
}

#[test]
fn test_word_portugal() {
    let regex = generate_regex("portugal");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("পর্তুগাল"));
}

#[test]
fn test_word_notification() {
    let regex = generate_regex("notification");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("নোটিফিকেশন"));
}

#[test]
fn test_word_tools() {
    let regex = generate_regex("tools");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("টুলস"));
}

#[test]
fn test_word_sports() {
    let regex = generate_regex("sports");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("স্পোর্টস"));
}

#[test]
fn test_word_speaker() {
    let regex = generate_regex("speaker");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("স্পিকার"));
}

#[test]
fn test_word_mechanic() {
    let regex = generate_regex("mechanic");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("মেকানিক"));
}

#[test]
fn test_word_brazil() {
    let regex = generate_regex("brazil");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ব্রাজিল"));
}

#[test]
fn test_word_everest() {
    let regex = generate_regex("everest");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("এভারেস্ট"));
}

#[test]
fn test_word_german() {
    let regex = generate_regex("german");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("জার্মান"));
}

#[test]
fn test_word_over() {
    let regex = generate_regex("over");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ওভার"));
}

#[test]
fn test_word_geography() {
    let regex = generate_regex("geography");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("জিওগ্রাফি"));
}

#[test]
fn test_word_scanner() {
    let regex = generate_regex("scanner");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("স্ক্যানার"));
}

#[test]
fn test_word_botany() {
    let regex = generate_regex("botany");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("বোটানি"));
}

#[test]
fn test_word_photo() {
    let regex = generate_regex("photo");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ফটো"));
}

#[test]
fn test_word_mouth() {
    let regex = generate_regex("mouth");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("মাউথ"));
}

#[test]
fn test_word_display() {
    let regex = generate_regex("display");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ডিসপ্লে"));
}

#[test]
fn test_word_bus() {
    let regex = generate_regex("bus");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("বাস"));
}

#[test]
fn test_word_local() {
    let regex = generate_regex("local");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("লোকাল"));
}

#[test]
fn test_word_patel() {
    let regex = generate_regex("patel");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("প্যাটেল"));
}

#[test]
fn test_word_weight() {
    let regex = generate_regex("weight");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ওয়েট"));
}

#[test]
fn test_word_gallery() {
    let regex = generate_regex("gallery");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("গ্যালারি"));
}

#[test]
fn test_word_album() {
    let regex = generate_regex("album");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("অ্যালবাম"));
}

#[test]
fn test_word_phonetic() {
    let regex = generate_regex("phonetic");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ফনেটিক"));
}

#[test]
fn test_word_setting() {
    let regex = generate_regex("setting");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("সেটিং"));
}

#[test]
fn test_word_drug() {
    let regex = generate_regex("drug");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ড্রাগ"));
}

#[test]
fn test_word_carrier() {
    let regex = generate_regex("carrier");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ক্যারিয়ার"));
}

#[test]
fn test_word_diesel() {
    let regex = generate_regex("diesel");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ডিজেল"));
}

#[test]
fn test_word_mill() {
    let regex = generate_regex("mill");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("মিল"));
}

#[test]
fn test_word_magistrate() {
    let regex = generate_regex("magistrate");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ম্যাজিস্ট্রেট"));
}

#[test]
fn test_word_comment() {
    let regex = generate_regex("comment");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("কমেন্ট"));
}

#[test]
fn test_word_offic() {
    let regex = generate_regex("offic");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("অফিস"));
}

#[test]
fn test_word_scooter() {
    let regex = generate_regex("scooter");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("স্কুটার"));
}

#[test]
fn test_word_marx() {
    let regex = generate_regex("marx");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("মার্ক্স"));
}

#[test]
fn test_word_cookie() {
    let regex = generate_regex("cookie");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("কুকি"));
}

#[test]
fn test_word_scroll() {
    let regex = generate_regex("scroll");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("স্ক্রল"));
}

#[test]
fn test_word_kitchen() {
    let regex = generate_regex("kitchen");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("কিচেন"));
}

#[test]
fn test_word_fellen() {
    let regex = generate_regex("fellen");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ফেললেন"));
}

#[test]
fn test_word_london() {
    let regex = generate_regex("london");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("লন্ডন"));
}

#[test]
fn test_word_pirate() {
    let regex = generate_regex("pirate");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("পাইরেট"));
}

#[test]
fn test_word_telephone() {
    let regex = generate_regex("telephone");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("টেলিফোন"));
}

#[test]
fn test_word_authority() {
    let regex = generate_regex("authority");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("অথরিটি"));
}

#[test]
fn test_word_commercial() {
    let regex = generate_regex("commercial");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("কমার্শিয়াল"));
}

#[test]
fn test_word_idiot() {
    let regex = generate_regex("idiot");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ইডিয়ট"));
}

#[test]
fn test_word_military() {
    let regex = generate_regex("military");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("মিলিটারি"));
}

#[test]
fn test_word_cricketer() {
    let regex = generate_regex("cricketer");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ক্রিকেটার"));
}

#[test]
fn test_word_definite() {
    let regex = generate_regex("definite");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ডেফিনিট"));
}

#[test]
fn test_word_prepaid() {
    let regex = generate_regex("prepaid");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("প্রিপেড"));
}

#[test]
fn test_word_register() {
    let regex = generate_regex("register");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("রেজিস্টার"));
}

#[test]
fn test_word_nurse() {
    let regex = generate_regex("nurse");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("নার্স"));
}

#[test]
fn test_word_shift() {
    let regex = generate_regex("shift");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("শিফট"));
}

#[test]
fn test_word_testimonial() {
    let regex = generate_regex("testimonial");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("টেস্টিমোনিয়াল"));
}

#[test]
fn test_word_string() {
    let regex = generate_regex("string");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("স্ট্রিং"));
}

#[test]
fn test_word_desktop() {
    let regex = generate_regex("desktop");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ডেস্কটপ"));
}

#[test]
fn test_word_dairy() {
    let regex = generate_regex("dairy");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ডেয়ারি"));
}

#[test]
fn test_word_lawyer() {
    let regex = generate_regex("lawyer");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("লইয়ার"));
}

#[test]
fn test_word_email() {
    let regex = generate_regex("email");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ইমেইল"));
}

#[test]
fn test_word_airport() {
    let regex = generate_regex("airport");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("এয়ারপোর্ট"));
}

#[test]
fn test_word_dialog() {
    let regex = generate_regex("dialog");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ডায়লগ"));
}

#[test]
fn test_word_blue() {
    let regex = generate_regex("blue");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ব্লু"));
}

#[test]
fn test_word_england() {
    let regex = generate_regex("england");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ইংল্যান্ড"));
}

#[test]
fn test_word_open() {
    let regex = generate_regex("open");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ওপেন"));
}

#[test]
fn test_word_debit() {
    let regex = generate_regex("debit");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ডেবিট"));
}

#[test]
fn test_word_cassette() {
    let regex = generate_regex("cassette");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ক্যাসেট"));
}

#[test]
fn test_word_call() {
    let regex = generate_regex("call");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("কল"));
}

#[test]
fn test_word_harry() {
    let regex = generate_regex("harry");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("হ্যারি"));
}

#[test]
fn test_word_violin() {
    let regex = generate_regex("violin");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ভায়োলিন"));
}

#[test]
fn test_word_ounce() {
    let regex = generate_regex("ounce");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("আউন্স"));
}

#[test]
fn test_word_dentist() {
    let regex = generate_regex("dentist");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ডেন্টিস্ট"));
}

#[test]
fn test_word_hitler() {
    let regex = generate_regex("hitler");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("হিটলার"));
}

#[test]
fn test_word_superman() {
    let regex = generate_regex("superman");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("সুপারম্যান"));
}

#[test]
fn test_word_microbiology() {
    let regex = generate_regex("microbiology");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("মাইক্রোবায়োলজি"));
}

#[test]
fn test_word_twitter() {
    let regex = generate_regex("twitter");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("টুইটার"));
}

#[test]
fn test_word_fashion() {
    let regex = generate_regex("fashion");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ফ্যাশান"));
}

#[test]
fn test_word_express() {
    let regex = generate_regex("express");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("এক্সপ্রেস"));
}

#[test]
fn test_word_legal() {
    let regex = generate_regex("legal");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("লিগাল"));
}

#[test]
fn test_word_battery() {
    let regex = generate_regex("battery");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ব্যাটারি"));
}

#[test]
fn test_word_suite() {
    let regex = generate_regex("suite");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("স্যুট"));
}

#[test]
fn test_word_current() {
    let regex = generate_regex("current");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("কারেন্ট"));
}

#[test]
fn test_word_chamber() {
    let regex = generate_regex("chamber");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("চেম্বার"));
}

#[test]
fn test_word_registration() {
    let regex = generate_regex("registration");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("রেজিস্ট্রেশন"));
}

#[test]
fn test_word_update() {
    let regex = generate_regex("update");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("আপডেট"));
}

#[test]
fn test_word_browser() {
    let regex = generate_regex("browser");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ব্রাউজার"));
}

#[test]
fn test_word_cadre() {
    let regex = generate_regex("cadre");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ক্যাডার"));
}

#[test]
fn test_word_forum() {
    let regex = generate_regex("forum");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ফোরাম"));
}

#[test]
fn test_word_kingdom() {
    let regex = generate_regex("kingdom");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("কিংডম"));
}

#[test]
fn test_word_bismillah() {
    let regex = generate_regex("bismillah");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("বিসমিল্লাহ"));
}

#[test]
fn test_word_english() {
    let regex = generate_regex("english");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ইংলিশ"));
}

#[test]
fn test_word_chicken() {
    let regex = generate_regex("chicken");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("চিকেন"));
}

#[test]
fn test_word_swiss() {
    let regex = generate_regex("swiss");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("সুইস"));
}

#[test]
fn test_word_news() {
    let regex = generate_regex("news");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("নিউজ"));
}

#[test]
fn test_word_memory() {
    let regex = generate_regex("memory");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("মেমোরি"));
}

#[test]
fn test_word_surf() {
    let regex = generate_regex("surf");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("সার্ফ"));
}

#[test]
fn test_word_dissection() {
    let regex = generate_regex("dissection");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ডিসেকশন"));
}

#[test]
fn test_word_minimize() {
    let regex = generate_regex("minimize");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("মিনিমাইজ"));
}

#[test]
fn test_word_port() {
    let regex = generate_regex("port");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("পোর্ট"));
}

#[test]
fn test_word_minimum() {
    let regex = generate_regex("minimum");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("মিনিমাম"));
}

#[test]
fn test_word_tube() {
    let regex = generate_regex("tube");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("টিউব"));
}

#[test]
fn test_word_lounge() {
    let regex = generate_regex("lounge");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("লাউঞ্জ"));
}

#[test]
fn test_word_bengali() {
    let regex = generate_regex("bengali");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("বেঙ্গলি"));
}

#[test]
fn test_word_titanic() {
    let regex = generate_regex("titanic");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("টাইটানিক"));
}

#[test]
fn test_word_electrical() {
    let regex = generate_regex("electrical");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ইলেক্ট্রিক্যাল"));
}

#[test]
fn test_word_avenue() {
    let regex = generate_regex("avenue");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("অ্যাভেন্যু"));
}

#[test]
fn test_word_threat() {
    let regex = generate_regex("threat");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("থ্রেট"));
}

#[test]
fn test_word_app() {
    let regex = generate_regex("app");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("অ্যাপ"));
}

#[test]
fn test_word_location() {
    let regex = generate_regex("location");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("লোকেশন"));
}

#[test]
fn test_word_sexy() {
    let regex = generate_regex("sexy");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("সেক্সি"));
}

#[test]
fn test_word_nobel() {
    let regex = generate_regex("nobel");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("নোবেল"));
}

#[test]
fn test_word_lawrence() {
    let regex = generate_regex("lawrence");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("লরেন্স"));
}

#[test]
fn test_word_delivery() {
    let regex = generate_regex("delivery");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ডেলিভারি"));
}

#[test]
fn test_word_theater() {
    let regex = generate_regex("theater");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("থিয়েটার"));
}

#[test]
fn test_word_tyre() {
    let regex = generate_regex("tyre");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("টায়ার"));
}

#[test]
fn test_word_adapter() {
    let regex = generate_regex("adapter");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("অ্যাডাপ্টার"));
}

#[test]
fn test_word_notebook() {
    let regex = generate_regex("notebook");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("নোটবুক"));
}

#[test]
fn test_word_fuzzy() {
    let regex = generate_regex("fuzzy");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ফাজি"));
}

#[test]
fn test_word_airways() {
    let regex = generate_regex("airways");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("এয়ারওয়েজ"));
}

#[test]
fn test_word_click() {
    let regex = generate_regex("click");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ক্লিক"));
}

#[test]
fn test_word_activity() {
    let regex = generate_regex("activity");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("অ্যাক্টিভিটি"));
}

#[test]
fn test_word_medicine() {
    let regex = generate_regex("medicine");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("মেডিসিন"));
}

#[test]
fn test_word_argentina() {
    let regex = generate_regex("argentina");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("আর্জেন্টিনা"));
}

#[test]
fn test_word_rubber() {
    let regex = generate_regex("rubber");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("রাবার"));
}

#[test]
fn test_word_out() {
    let regex = generate_regex("out");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("আউট"));
}

#[test]
fn test_word_building() {
    let regex = generate_regex("building");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("বিল্ডিং"));
}

#[test]
fn test_word_glass() {
    let regex = generate_regex("glass");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("গ্লাস"));
}

#[test]
fn test_word_pregnant() {
    let regex = generate_regex("pregnant");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("প্রেগন্যান্ট"));
}

#[test]
fn test_word_include() {
    let regex = generate_regex("include");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ইনক্লুড"));
}

#[test]
fn test_word_plug() {
    let regex = generate_regex("plug");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("প্লাগ"));
}

#[test]
fn test_word_shanghai() {
    let regex = generate_regex("shanghai");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("সাংহাই"));
}

#[test]
fn test_word_micro() {
    let regex = generate_regex("micro");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("মাইক্রো"));
}

#[test]
fn test_word_insure() {
    let regex = generate_regex("insure");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ইন্সিওর"));
}

#[test]
fn test_word_cursor() {
    let regex = generate_regex("cursor");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("কার্সর"));
}

#[test]
fn test_word_inches() {
    let regex = generate_regex("inches");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ইঞ্চি"));
}

#[test]
fn test_word_wait() {
    let regex = generate_regex("wait");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ওয়েট"));
}

#[test]
fn test_word_euro() {
    let regex = generate_regex("euro");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ইউরো"));
}

#[test]
fn test_word_detective() {
    let regex = generate_regex("detective");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ডিটেক্টিভ"));
}

#[test]
fn test_word_wine() {
    let regex = generate_regex("wine");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ওয়াইন"));
}

#[test]
fn test_word_monitor() {
    let regex = generate_regex("monitor");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("মনিটর"));
}

#[test]
fn test_word_small() {
    let regex = generate_regex("small");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("স্মল"));
}

#[test]
fn test_word_event() {
    let regex = generate_regex("event");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ইভেন্ট"));
}

#[test]
fn test_word_control() {
    let regex = generate_regex("control");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("কন্ট্রোল"));
}

#[test]
fn test_word_island() {
    let regex = generate_regex("island");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("আইল্যান্ড"));
}

#[test]
fn test_word_warming() {
    let regex = generate_regex("warming");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ওয়ার্মিং"));
}

#[test]
fn test_word_carry() {
    let regex = generate_regex("carry");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ক্যারি"));
}

#[test]
fn test_word_documents() {
    let regex = generate_regex("documents");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ডকুমেন্টস"));
}

#[test]
fn test_word_tourist() {
    let regex = generate_regex("tourist");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ট্যুরিস্ট"));
}

#[test]
fn test_word_ballot() {
    let regex = generate_regex("ballot");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ব্যালট"));
}

#[test]
fn test_word_haj() {
    let regex = generate_regex("haj");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("হজ"));
}

#[test]
fn test_word_victoria() {
    let regex = generate_regex("victoria");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ভিক্টোরিয়া"));
}

#[test]
fn test_word_islam() {
    let regex = generate_regex("islam");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ইসলাম"));
}

#[test]
fn test_word_saint() {
    let regex = generate_regex("saint");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("সেন্ট"));
}

#[test]
fn test_word_robot() {
    let regex = generate_regex("robot");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("রোবট"));
}

#[test]
fn test_word_human() {
    let regex = generate_regex("human");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("হিউম্যান"));
}

#[test]
fn test_word_friend() {
    let regex = generate_regex("friend");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ফ্রেন্ড"));
}

#[test]
fn test_word_february() {
    let regex = generate_regex("february");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ফেব্রুয়ারি"));
}

#[test]
fn test_word_type() {
    let regex = generate_regex("type");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("টাইপ"));
}

#[test]
fn test_word_nylon() {
    let regex = generate_regex("nylon");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("নাইলন"));
}

#[test]
fn test_word_tax() {
    let regex = generate_regex("tax");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ট্যাক্স"));
}

#[test]
fn test_word_cricket() {
    let regex = generate_regex("cricket");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ক্রিকেট"));
}

#[test]
fn test_word_page() {
    let regex = generate_regex("page");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("পেইজ"));
}

#[test]
fn test_word_charge() {
    let regex = generate_regex("charge");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("চার্জ"));
}

#[test]
fn test_word_lane() {
    let regex = generate_regex("lane");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("লেন"));
}

#[test]
fn test_word_logical() {
    let regex = generate_regex("logical");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("লজিকাল"));
}

#[test]
fn test_word_style() {
    let regex = generate_regex("style");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("স্টাইল"));
}

#[test]
fn test_word_world() {
    let regex = generate_regex("world");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ওয়ার্ল্ড"));
}

#[test]
fn test_word_electronic() {
    let regex = generate_regex("electronic");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ইলেক্ট্রনিক"));
}

#[test]
fn test_word_standard() {
    let regex = generate_regex("standard");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("স্ট্যান্ডার্ড"));
}

#[test]
fn test_word_alt() {
    let regex = generate_regex("alt");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("অল্ট"));
}

#[test]
fn test_word_automobile() {
    let regex = generate_regex("automobile");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("অটোমোবাইল"));
}

#[test]
fn test_word_screen() {
    let regex = generate_regex("screen");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("স্ক্রিন"));
}

#[test]
fn test_word_camera() {
    let regex = generate_regex("camera");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ক্যামেরা"));
}

#[test]
fn test_word_television() {
    let regex = generate_regex("television");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("টেলিভিশন"));
}

#[test]
fn test_word_sir() {
    let regex = generate_regex("sir");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("স্যার"));
}

#[test]
fn test_word_configuration() {
    let regex = generate_regex("configuration");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("কনফিগারেশন"));
}

#[test]
fn test_word_row() {
    let regex = generate_regex("row");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("রো"));
}

#[test]
fn test_word_network() {
    let regex = generate_regex("network");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("নেটওয়ার্ক"));
}

#[test]
fn test_word_medium() {
    let regex = generate_regex("medium");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("মিডিয়াম"));
}

#[test]
fn test_word_journalism() {
    let regex = generate_regex("journalism");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("জার্নালিজম"));
}

#[test]
fn test_word_steering() {
    let regex = generate_regex("steering");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("স্টিয়ারিং"));
}

#[test]
fn test_word_time() {
    let regex = generate_regex("time");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("টাইম"));
}

#[test]
fn test_word_feed() {
    let regex = generate_regex("feed");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ফিড"));
}

#[test]
fn test_word_calcutta() {
    let regex = generate_regex("calcutta");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ক্যালকাটা"));
}

#[test]
fn test_word_certificate() {
    let regex = generate_regex("certificate");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("সার্টিফিকেট"));
}

#[test]
fn test_word_plus() {
    let regex = generate_regex("plus");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("প্লাস"));
}

#[test]
fn test_word_tool() {
    let regex = generate_regex("tool");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("টুল"));
}

#[test]
fn test_word_group() {
    let regex = generate_regex("group");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("গ্রুপ"));
}

#[test]
fn test_word_mutton() {
    let regex = generate_regex("mutton");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("মাটন"));
}

#[test]
fn test_word_adobe() {
    let regex = generate_regex("adobe");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("অ্যাডোব"));
}

#[test]
fn test_word_aids() {
    let regex = generate_regex("aids");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("এইডস"));
}

#[test]
fn test_word_definition() {
    let regex = generate_regex("definition");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ডেফিনেশন"));
}

#[test]
fn test_word_sweater() {
    let regex = generate_regex("sweater");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("সোয়েটার"));
}

#[test]
fn test_word_computer() {
    let regex = generate_regex("computer");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("কম্পিউটার"));
}

#[test]
fn test_word_academy() {
    let regex = generate_regex("academy");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("অ্যাকাডেমি"));
}

#[test]
fn test_word_info() {
    let regex = generate_regex("info");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ইনফো"));
}

#[test]
fn test_word_mess() {
    let regex = generate_regex("mess");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("মেস"));
}

#[test]
fn test_word_unit() {
    let regex = generate_regex("unit");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ইউনিট"));
}

#[test]
fn test_word_extension() {
    let regex = generate_regex("extension");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("এক্সটেনশন"));
}

#[test]
fn test_word_whisky() {
    let regex = generate_regex("whisky");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("হুইস্কি"));
}

#[test]
fn test_word_auto() {
    let regex = generate_regex("auto");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("অটো"));
}

#[test]
fn test_word_cream() {
    let regex = generate_regex("cream");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ক্রিম"));
}

#[test]
fn test_word_draft() {
    let regex = generate_regex("draft");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ড্রাফট"));
}

#[test]
fn test_word_blackberry() {
    let regex = generate_regex("blackberry");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ব্ল্যাকবেরি"));
}

#[test]
fn test_word_shock() {
    let regex = generate_regex("shock");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("শক"));
}

#[test]
fn test_word_layout() {
    let regex = generate_regex("layout");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("লেআউট"));
}

#[test]
fn test_word_microscope() {
    let regex = generate_regex("microscope");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("মাইক্রোস্কোপ"));
}

#[test]
fn test_word_trapezium() {
    let regex = generate_regex("trapezium");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ট্রাপিজিয়াম"));
}

#[test]
fn test_word_break() {
    let regex = generate_regex("break");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ব্রেক"));
}

#[test]
fn test_word_sauce() {
    let regex = generate_regex("sauce");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("সস"));
}

#[test]
fn test_word_crystal() {
    let regex = generate_regex("crystal");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ক্রিস্টাল"));
}

#[test]
fn test_word_reader() {
    let regex = generate_regex("reader");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("রিডার"));
}

#[test]
fn test_word_alphabet() {
    let regex = generate_regex("alphabet");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("অ্যালফাবেট"));
}

#[test]
fn test_word_mercedes() {
    let regex = generate_regex("mercedes");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("মার্সিডিজ"));
}

#[test]
fn test_word_iron() {
    let regex = generate_regex("iron");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("আয়রন"));
}

#[test]
fn test_word_team() {
    let regex = generate_regex("team");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("টিম"));
}

#[test]
fn test_word_cross() {
    let regex = generate_regex("cross");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ক্রস"));
}

#[test]
fn test_word_maths() {
    let regex = generate_regex("maths");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ম্যাথস"));
}

#[test]
fn test_word_delete() {
    let regex = generate_regex("delete");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ডিলিট"));
}

#[test]
fn test_word_sanitary() {
    let regex = generate_regex("sanitary");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("স্যানিটারি"));
}

#[test]
fn test_word_class() {
    let regex = generate_regex("class");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ক্লাস"));
}

#[test]
fn test_word_director() {
    let regex = generate_regex("director");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ডিরেক্টর"));
}

#[test]
fn test_word_president() {
    let regex = generate_regex("president");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("প্রেসিডেন্ট"));
}

#[test]
fn test_word_market() {
    let regex = generate_regex("market");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("মার্কেট"));
}

#[test]
fn test_word_volume() {
    let regex = generate_regex("volume");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ভলিউম"));
}

#[test]
fn test_word_overhead() {
    let regex = generate_regex("overhead");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ওভারহেড"));
}

#[test]
fn test_word_antivirus() {
    let regex = generate_regex("antivirus");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("অ্যান্টিভাইরাস"));
}

#[test]
fn test_word_easy() {
    let regex = generate_regex("easy");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ইজি"));
}

#[test]
fn test_word_officer() {
    let regex = generate_regex("officer");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("অফিসার"));
}

#[test]
fn test_word_clear() {
    let regex = generate_regex("clear");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ক্লিয়ার"));
}

#[test]
fn test_word_sound() {
    let regex = generate_regex("sound");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("সাউন্ড"));
}

#[test]
fn test_word_byte() {
    let regex = generate_regex("byte");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("বাইট"));
}

#[test]
fn test_word_operation() {
    let regex = generate_regex("operation");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("অপারেশন"));
}

#[test]
fn test_word_access() {
    let regex = generate_regex("access");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("অ্যাক্সেস"));
}

#[test]
fn test_word_china() {
    let regex = generate_regex("china");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("চায়না"));
}

#[test]
fn test_word_message() {
    let regex = generate_regex("message");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("মেসেজ"));
}

#[test]
fn test_word_school() {
    let regex = generate_regex("school");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("স্কুল"));
}

#[test]
fn test_word_united() {
    let regex = generate_regex("united");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ইউনাইটেড"));
}

#[test]
fn test_word_rights() {
    let regex = generate_regex("rights");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("রাইটস"));
}

#[test]
fn test_word_baseball() {
    let regex = generate_regex("baseball");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("বেসবল"));
}

#[test]
fn test_word_luxury() {
    let regex = generate_regex("luxury");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("লাক্সারি"));
}

#[test]
fn test_word_tab() {
    let regex = generate_regex("tab");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ট্যাব"));
}

#[test]
fn test_word_tray() {
    let regex = generate_regex("tray");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ট্রে"));
}

#[test]
fn test_word_tonic() {
    let regex = generate_regex("tonic");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("টনিক"));
}

#[test]
fn test_word_avast() {
    let regex = generate_regex("avast");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("অ্যাভাস্ট"));
}

#[test]
fn test_word_november() {
    let regex = generate_regex("november");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("নভেম্বর"));
}

#[test]
fn test_word_with() {
    let regex = generate_regex("with");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("উইথ"));
}

#[test]
fn test_word_city() {
    let regex = generate_regex("city");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("সিটি"));
}

#[test]
fn test_word_fiction() {
    let regex = generate_regex("fiction");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ফিকশন"));
}

#[test]
fn test_word_macintosh() {
    let regex = generate_regex("macintosh");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ম্যাকিনটোশ"));
}

#[test]
fn test_word_start() {
    let regex = generate_regex("start");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("স্টার্ট"));
}

#[test]
fn test_word_archive() {
    let regex = generate_regex("archive");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("আর্কাইভ"));
}

#[test]
fn test_word_father() {
    let regex = generate_regex("father");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ফাদার"));
}

#[test]
fn test_word_music() {
    let regex = generate_regex("music");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("মিউজিক"));
}

#[test]
fn test_word_scotland() {
    let regex = generate_regex("scotland");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("স্কটল্যান্ড"));
}

#[test]
fn test_word_space() {
    let regex = generate_regex("space");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("স্পেস"));
}

#[test]
fn test_word_paediatric() {
    let regex = generate_regex("paediatric");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("পেডিয়াট্রিক"));
}

#[test]
fn test_word_press() {
    let regex = generate_regex("press");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("প্রেস"));
}

#[test]
fn test_word_logic() {
    let regex = generate_regex("logic");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("লজিক"));
}

#[test]
fn test_word_gates() {
    let regex = generate_regex("gates");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("গেটস"));
}

#[test]
fn test_word_pathology() {
    let regex = generate_regex("pathology");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("প্যাথলজি"));
}

#[test]
fn test_word_crack() {
    let regex = generate_regex("crack");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ক্র্যাক"));
}

#[test]
fn test_word_photocopy() {
    let regex = generate_regex("photocopy");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ফটোকপি"));
}

#[test]
fn test_word_yard() {
    let regex = generate_regex("yard");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ইয়ার্ড"));
}

#[test]
fn test_word_rugby() {
    let regex = generate_regex("rugby");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("রাগবি"));
}

#[test]
fn test_word_smart() {
    let regex = generate_regex("smart");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("স্মার্ট"));
}

#[test]
fn test_word_britain() {
    let regex = generate_regex("britain");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ব্রিটেন"));
}

#[test]
fn test_word_disk() {
    let regex = generate_regex("disk");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ডিস্ক"));
}

#[test]
fn test_word_block() {
    let regex = generate_regex("block");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ব্লক"));
}

#[test]
fn test_word_norway() {
    let regex = generate_regex("norway");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("নরওয়ে"));
}

#[test]
fn test_word_card() {
    let regex = generate_regex("card");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("কার্ড"));
}

#[test]
fn test_word_pant() {
    let regex = generate_regex("pant");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("প্যান্ট"));
}

#[test]
fn test_word_partition() {
    let regex = generate_regex("partition");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("পার্টিশন"));
}

#[test]
fn test_word_april() {
    let regex = generate_regex("april");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("এপ্রিল"));
}

#[test]
fn test_word_clipboard() {
    let regex = generate_regex("clipboard");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ক্লিপবোর্ড"));
}

#[test]
fn test_word_piracy() {
    let regex = generate_regex("piracy");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("পাইরেসি"));
}

#[test]
fn test_word_coal() {
    let regex = generate_regex("coal");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("কোল"));
}

#[test]
fn test_word_mall() {
    let regex = generate_regex("mall");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("মল"));
}

#[test]
fn test_word_electricity() {
    let regex = generate_regex("electricity");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ইলেক্ট্রিসিটি"));
}

#[test]
fn test_word_pound() {
    let regex = generate_regex("pound");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("পাউন্ড"));
}

#[test]
fn test_word_wheel() {
    let regex = generate_regex("wheel");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("হুইল"));
}

#[test]
fn test_word_morning() {
    let regex = generate_regex("morning");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("মর্নিং"));
}

#[test]
fn test_word_oxygen() {
    let regex = generate_regex("oxygen");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("অক্সিজেন"));
}

#[test]
fn test_word_lunch() {
    let regex = generate_regex("lunch");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("লাঞ্চ"));
}

#[test]
fn test_word_scholar() {
    let regex = generate_regex("scholar");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("স্কলার"));
}

#[test]
fn test_word_telescope() {
    let regex = generate_regex("telescope");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("টেলিস্কোপ"));
}

#[test]
fn test_word_hell() {
    let regex = generate_regex("hell");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("হেল"));
}

#[test]
fn test_word_refresh() {
    let regex = generate_regex("refresh");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("রিফ্রেশ"));
}

#[test]
fn test_word_account() {
    let regex = generate_regex("account");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("অ্যাকাউন্ট"));
}

#[test]
fn test_word_jeans() {
    let regex = generate_regex("jeans");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("জিনস"));
}

#[test]
fn test_word_native() {
    let regex = generate_regex("native");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("নেটিভ"));
}

#[test]
fn test_word_spider() {
    let regex = generate_regex("spider");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("স্পাইডার"));
}

#[test]
fn test_word_october() {
    let regex = generate_regex("october");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("অক্টোবর"));
}

#[test]
fn test_word_helper() {
    let regex = generate_regex("helper");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("হেল্পার"));
}

#[test]
fn test_word_commerce() {
    let regex = generate_regex("commerce");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("কমার্স"));
}

#[test]
fn test_word_july() {
    let regex = generate_regex("july");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("জুলাই"));
}

#[test]
fn test_word_slide() {
    let regex = generate_regex("slide");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("স্লাইড"));
}

#[test]
fn test_word_gunda() {
    let regex = generate_regex("gunda");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("গুন্ডা"));
}

#[test]
fn test_word_channel() {
    let regex = generate_regex("channel");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("চ্যানেল"));
}

#[test]
fn test_word_december() {
    let regex = generate_regex("december");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ডিসেম্বর"));
}

#[test]
fn test_word_options() {
    let regex = generate_regex("options");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("অপশনস"));
}

#[test]
fn test_word_tourism() {
    let regex = generate_regex("tourism");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ট্যুরিজম"));
}

#[test]
fn test_word_fund() {
    let regex = generate_regex("fund");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ফান্ড"));
}

#[test]
fn test_word_catalog() {
    let regex = generate_regex("catalog");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ক্যাটালগ"));
}

#[test]
fn test_word_restaurant() {
    let regex = generate_regex("restaurant");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("রেস্টুরেন্ট"));
}

#[test]
fn test_word_community() {
    let regex = generate_regex("community");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("কমিউনিটি"));
}

#[test]
fn test_word_copy() {
    let regex = generate_regex("copy");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("কপি"));
}

#[test]
fn test_word_inch() {
    let regex = generate_regex("inch");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ইঞ্চি"));
}

#[test]
fn test_word_laptop() {
    let regex = generate_regex("laptop");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ল্যাপটপ"));
}

#[test]
fn test_word_singapore() {
    let regex = generate_regex("singapore");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("সিঙ্গাপুর"));
}

#[test]
fn test_word_circular() {
    let regex = generate_regex("circular");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("সার্কুলার"));
}

#[test]
fn test_word_edition() {
    let regex = generate_regex("edition");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("এডিশন"));
}

#[test]
fn test_word_hollywood() {
    let regex = generate_regex("hollywood");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("হলিউড"));
}

#[test]
fn test_word_pharmacology() {
    let regex = generate_regex("pharmacology");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ফার্মাকোলজি"));
}

#[test]
fn test_word_dictionary() {
    let regex = generate_regex("dictionary");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ডিকশনারি"));
}

#[test]
fn test_word_chair() {
    let regex = generate_regex("chair");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("চেয়ার"));
}

#[test]
fn test_word_sweety() {
    let regex = generate_regex("sweety");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("সুইটি"));
}

#[test]
fn test_word_george() {
    let regex = generate_regex("george");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("জর্জ"));
}

#[test]
fn test_word_calculator() {
    let regex = generate_regex("calculator");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ক্যালকুলেটর"));
}

#[test]
fn test_word_catalogue() {
    let regex = generate_regex("catalogue");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ক্যাটালগ"));
}

#[test]
fn test_word_paediatrics() {
    let regex = generate_regex("paediatrics");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("পেডিয়াট্রিক্স"));
}

#[test]
fn test_word_badminton() {
    let regex = generate_regex("badminton");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ব্যাডমিন্টন"));
}

#[test]
fn test_word_earth() {
    let regex = generate_regex("earth");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("আর্থ"));
}

#[test]
fn test_word_board() {
    let regex = generate_regex("board");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("বোর্ড"));
}

#[test]
fn test_word_capacity() {
    let regex = generate_regex("capacity");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ক্যাপাসিটি"));
}

#[test]
fn test_word_acer() {
    let regex = generate_regex("acer");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("এসার"));
}

#[test]
fn test_word_committee() {
    let regex = generate_regex("committee");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("কমিটি"));
}

#[test]
fn test_word_copyright() {
    let regex = generate_regex("copyright");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("কপিরাইট"));
}

#[test]
fn test_word_council() {
    let regex = generate_regex("council");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("কাউন্সিল"));
}

#[test]
fn test_word_good() {
    let regex = generate_regex("good");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("গুড"));
}

#[test]
fn test_word_army() {
    let regex = generate_regex("army");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("আর্মি"));
}

#[test]
fn test_word_original() {
    let regex = generate_regex("original");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("অরিজিনাল"));
}

#[test]
fn test_word_character() {
    let regex = generate_regex("character");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ক্যারেক্টার"));
}

#[test]
fn test_word_session() {
    let regex = generate_regex("session");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("সেশন"));
}

#[test]
fn test_word_cent() {
    let regex = generate_regex("cent");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("সেন্ট"));
}

#[test]
fn test_word_information() {
    let regex = generate_regex("information");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ইনফর্মেশন"));
}

#[test]
fn test_word_engineer() {
    let regex = generate_regex("engineer");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ইঞ্জিনিয়ার"));
}

#[test]
fn test_word_congress() {
    let regex = generate_regex("congress");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("কংগ্রেস"));
}

#[test]
fn test_word_movie() {
    let regex = generate_regex("movie");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("মুভি"));
}

#[test]
fn test_word_patent() {
    let regex = generate_regex("patent");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("পেটেন্ট"));
}

#[test]
fn test_word_keyboard() {
    let regex = generate_regex("keyboard");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("কিবোর্ড"));
}

#[test]
fn test_word_town() {
    let regex = generate_regex("town");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("টাউন"));
}

#[test]
fn test_word_plane() {
    let regex = generate_regex("plane");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("প্লেন"));
}

#[test]
fn test_word_final() {
    let regex = generate_regex("final");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ফাইনাল"));
}

#[test]
fn test_word_rape() {
    let regex = generate_regex("rape");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("রেপ"));
}

#[test]
fn test_word_set() {
    let regex = generate_regex("set");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("সেট"));
}

#[test]
fn test_word_user() {
    let regex = generate_regex("user");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ইউজার"));
}

#[test]
fn test_word_tsunami() {
    let regex = generate_regex("tsunami");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("সুনামি"));
}

#[test]
fn test_word_status() {
    let regex = generate_regex("status");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("স্ট্যাটাস"));
}

#[test]
fn test_word_property() {
    let regex = generate_regex("property");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("প্রপার্টি"));
}

#[test]
fn test_word_union() {
    let regex = generate_regex("union");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ইউনিয়ন"));
}

#[test]
fn test_word_angle() {
    let regex = generate_regex("angle");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("অ্যাঙ্গেল"));
}

#[test]
fn test_word_coach() {
    let regex = generate_regex("coach");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("কোচ"));
}

#[test]
fn test_word_plate() {
    let regex = generate_regex("plate");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("প্লেট"));
}

#[test]
fn test_word_emirates() {
    let regex = generate_regex("emirates");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("এমিরেটস"));
}

#[test]
fn test_word_dialogue() {
    let regex = generate_regex("dialogue");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ডায়লগ"));
}

#[test]
fn test_word_stock() {
    let regex = generate_regex("stock");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("স্টক"));
}

#[test]
fn test_word_prize() {
    let regex = generate_regex("prize");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("প্রাইজ"));
}

#[test]
fn test_word_january() {
    let regex = generate_regex("january");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("জানুয়ারি"));
}

#[test]
fn test_word_stand() {
    let regex = generate_regex("stand");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("স্ট্যান্ড"));
}

#[test]
fn test_word_recent() {
    let regex = generate_regex("recent");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("রিসেন্ট"));
}

#[test]
fn test_word_apply() {
    let regex = generate_regex("apply");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("অ্যাপ্লাই"));
}

#[test]
fn test_word_caps() {
    let regex = generate_regex("caps");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ক্যাপস"));
}

#[test]
fn test_word_alarm() {
    let regex = generate_regex("alarm");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("অ্যালার্ম"));
}

#[test]
fn test_word_point() {
    let regex = generate_regex("point");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("পয়েন্ট"));
}

#[test]
fn test_word_wireless() {
    let regex = generate_regex("wireless");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ওয়ারলেস"));
}

#[test]
fn test_word_india() {
    let regex = generate_regex("india");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ইন্ডিয়া"));
}

#[test]
fn test_word_project() {
    let regex = generate_regex("project");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("প্রোজেক্ট"));
}

#[test]
fn test_word_museum() {
    let regex = generate_regex("museum");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("মিউজিয়াম"));
}

#[test]
fn test_word_remote() {
    let regex = generate_regex("remote");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("রিমোট"));
}

#[test]
fn test_word_diamond() {
    let regex = generate_regex("diamond");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ডায়মন্ড"));
}

#[test]
fn test_word_zaman() {
    let regex = generate_regex("zaman");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("জামান"));
}

#[test]
fn test_word_stool() {
    let regex = generate_regex("stool");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("স্টুল"));
}

#[test]
fn test_word_new() {
    let regex = generate_regex("new");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("নিউ"));
}

#[test]
fn test_word_orange() {
    let regex = generate_regex("orange");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("অরেঞ্জ"));
}

#[test]
fn test_word_show() {
    let regex = generate_regex("show");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("শো"));
}

#[test]
fn test_word_parker() {
    let regex = generate_regex("parker");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("পার্কার"));
}

#[test]
fn test_word_rail() {
    let regex = generate_regex("rail");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("রেল"));
}

#[test]
fn test_word_joint() {
    let regex = generate_regex("joint");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("জয়েন্ট"));
}

#[test]
fn test_word_diary() {
    let regex = generate_regex("diary");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ডায়েরি"));
}

#[test]
fn test_word_domain() {
    let regex = generate_regex("domain");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ডোমেইন"));
}

#[test]
fn test_word_painting() {
    let regex = generate_regex("painting");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("পেইন্টিং"));
}

#[test]
fn test_word_einstein() {
    let regex = generate_regex("einstein");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("আইন্সটাইন"));
}

#[test]
fn test_word_gynecology() {
    let regex = generate_regex("gynecology");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("গাইনেকোলজি"));
}

#[test]
fn test_word_tower() {
    let regex = generate_regex("tower");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("টাওয়ার"));
}

#[test]
fn test_word_century() {
    let regex = generate_regex("century");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("সেঞ্চুরি"));
}

#[test]
fn test_word_stamp() {
    let regex = generate_regex("stamp");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("স্ট্যাম্প"));
}

#[test]
fn test_word_cuba() {
    let regex = generate_regex("cuba");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("কিউবা"));
}

#[test]
fn test_word_dead() {
    let regex = generate_regex("dead");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ডেড"));
}

#[test]
fn test_word_antenna() {
    let regex = generate_regex("antenna");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("অ্যান্টেনা"));
}

#[test]
fn test_word_member() {
    let regex = generate_regex("member");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("মেম্বার"));
}

#[test]
fn test_word_wellington() {
    let regex = generate_regex("wellington");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ওয়েলিংটন"));
}

#[test]
fn test_word_club() {
    let regex = generate_regex("club");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ক্লাব"));
}

#[test]
fn test_word_newspaper() {
    let regex = generate_regex("newspaper");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("নিউজপেপার"));
}

#[test]
fn test_word_nonsense() {
    let regex = generate_regex("nonsense");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ননসেন্স"));
}

#[test]
fn test_word_beat() {
    let regex = generate_regex("beat");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("বিট"));
}

#[test]
fn test_word_pencil() {
    let regex = generate_regex("pencil");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("পেন্সিল"));
}

#[test]
fn test_word_and() {
    let regex = generate_regex("and");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("অ্যান্ড"));
}

#[test]
fn test_word_hertz() {
    let regex = generate_regex("hertz");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("হার্জ"));
}

#[test]
fn test_word_hanger() {
    let regex = generate_regex("hanger");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("হ্যাঙ্গার"));
}

#[test]
fn test_word_magazine() {
    let regex = generate_regex("magazine");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ম্যাগাজিন"));
}

#[test]
fn test_word_tank() {
    let regex = generate_regex("tank");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ট্যাঙ্ক"));
}

#[test]
fn test_word_hall() {
    let regex = generate_regex("hall");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("হল"));
}

#[test]
fn test_word_temporary() {
    let regex = generate_regex("temporary");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("টেম্পোরারি"));
}

#[test]
fn test_word_hard() {
    let regex = generate_regex("hard");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("হার্ড"));
}

#[test]
fn test_word_brake() {
    let regex = generate_regex("brake");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ব্রেক"));
}

#[test]
fn test_word_lobby() {
    let regex = generate_regex("lobby");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("লবি"));
}

#[test]
fn test_word_urine() {
    let regex = generate_regex("urine");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ইউরিন"));
}

#[test]
fn test_word_physics() {
    let regex = generate_regex("physics");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ফিজিক্স"));
}

#[test]
fn test_word_explorer() {
    let regex = generate_regex("explorer");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("এক্সপ্লোরার"));
}

#[test]
fn test_word_software() {
    let regex = generate_regex("software");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("সফটওয়্যার"));
}

#[test]
fn test_word_election() {
    let regex = generate_regex("election");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ইলেকশন"));
}

#[test]
fn test_word_sonar() {
    let regex = generate_regex("sonar");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("সোনার"));
}

#[test]
fn test_word_note() {
    let regex = generate_regex("note");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("নোট"));
}

#[test]
fn test_word_line() {
    let regex = generate_regex("line");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("লাইন"));
}

#[test]
fn test_word_item() {
    let regex = generate_regex("item");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("আইটেম"));
}

#[test]
fn test_word_define() {
    let regex = generate_regex("define");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ডিফাইন"));
}

#[test]
fn test_word_suitcase() {
    let regex = generate_regex("suitcase");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("সুটকেস"));
}

#[test]
fn test_word_association() {
    let regex = generate_regex("association");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("অ্যাসোসিয়েশন"));
}

#[test]
fn test_word_end() {
    let regex = generate_regex("end");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("এন্ড"));
}

#[test]
fn test_word_science() {
    let regex = generate_regex("science");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("সায়েন্স"));
}

#[test]
fn test_word_national() {
    let regex = generate_regex("national");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ন্যাশনাল"));
}

#[test]
fn test_word_mail() {
    let regex = generate_regex("mail");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("মেইল"));
}

#[test]
fn test_word_metric() {
    let regex = generate_regex("metric");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("মেট্রিক"));
}

#[test]
fn test_word_background() {
    let regex = generate_regex("background");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ব্যাকগ্রাউন্ড"));
}

#[test]
fn test_word_shampoo() {
    let regex = generate_regex("shampoo");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("শ্যাম্পু"));
}


#[test]
fn test_word_computing() {
    let regex = generate_regex("computing");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("কম্পিউটিং"));
}

#[test]
fn test_word_wallpaper() {
    let regex = generate_regex("wallpaper");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ওয়ালপেপার"));
}

#[test]
fn test_word_broadband() {
    let regex = generate_regex("broadband");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ব্রডব্যান্ড"));
}

#[test]
fn test_word_political() {
    let regex = generate_regex("political");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("পলিটিকাল"));
}

#[test]
fn test_word_route() {
    let regex = generate_regex("route");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("রুট"));
}

#[test]
fn test_word_nicobar() {
    let regex = generate_regex("nicobar");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("নিকোবার"));
}

#[test]
fn test_word_business() {
    let regex = generate_regex("business");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("বিজনেস"));
}

#[test]
fn test_word_forward() {
    let regex = generate_regex("forward");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ফরওয়ার্ড"));
}

#[test]
fn test_word_engine() {
    let regex = generate_regex("engine");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ইঞ্জিন"));
}

#[test]
fn test_word_greenland() {
    let regex = generate_regex("greenland");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("গ্রিনল্যান্ড"));
}

#[test]
fn test_word_palta() {
    let regex = generate_regex("palta");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("পাল্টা"));
}

#[test]
fn test_word_recycle() {
    let regex = generate_regex("recycle");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("রিসাইকেল"));
}

#[test]
fn test_word_east() {
    let regex = generate_regex("east");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ইস্ট"));
}

#[test]
fn test_word_signature() {
    let regex = generate_regex("signature");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("সিগনেচার"));
}

#[test]
fn test_word_birthday() {
    let regex = generate_regex("birthday");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("বার্থডে"));
}

#[test]
fn test_word_my() {
    let regex = generate_regex("my");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("মাই"));
}

#[test]
fn test_word_university() {
    let regex = generate_regex("university");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ইউনিভার্সিটি"));
}

#[test]
fn test_word_civil() {
    let regex = generate_regex("civil");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("সিভিল"));
}

#[test]
fn test_word_globalization() {
    let regex = generate_regex("globalization");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("গ্লোবালাইজেশন"));
}

#[test]
fn test_word_calendar() {
    let regex = generate_regex("calendar");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ক্যালেন্ডার"));
}

#[test]
fn test_word_chalk() {
    let regex = generate_regex("chalk");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("চক"));
}

#[test]
fn test_word_assistant() {
    let regex = generate_regex("assistant");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("অ্যাসিস্ট্যান্ট"));
}

#[test]
fn test_word_driver() {
    let regex = generate_regex("driver");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ড্রাইভার"));
}

#[test]
fn test_word_pulse() {
    let regex = generate_regex("pulse");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("পালস"));
}

#[test]
fn test_word_train() {
    let regex = generate_regex("train");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ট্রেন"));
}

#[test]
fn test_word_entry() {
    let regex = generate_regex("entry");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("এন্ট্রি"));
}

#[test]
fn test_word_source() {
    let regex = generate_regex("source");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("সোর্স"));
}

#[test]
fn test_word_september() {
    let regex = generate_regex("september");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("সেপ্টেম্বর"));
}

#[test]
fn test_word_article() {
    let regex = generate_regex("article");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("আর্টিকেল"));
}

#[test]
fn test_word_harmonium() {
    let regex = generate_regex("harmonium");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("হারমোনিয়াম"));
}

#[test]
fn test_word_professional() {
    let regex = generate_regex("professional");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("প্রোফেসনাল"));
}

#[test]
fn test_word_rhombus() {
    let regex = generate_regex("rhombus");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("রম্বস"));
}

#[test]
fn test_word_shirt() {
    let regex = generate_regex("shirt");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("শার্ট"));
}

#[test]
fn test_word_soldier() {
    let regex = generate_regex("soldier");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("সোলজার"));
}

#[test]
fn test_word_capture() {
    let regex = generate_regex("capture");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ক্যাপচার"));
}

#[test]
fn test_word_park() {
    let regex = generate_regex("park");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("পার্ক"));
}

#[test]
fn test_word_live() {
    let regex = generate_regex("live");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("লাইভ"));
}

#[test]
fn test_word_repair() {
    let regex = generate_regex("repair");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("রিপেয়ার"));
}

#[test]
fn test_word_surgery() {
    let regex = generate_regex("surgery");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("সার্জারি"));
}

#[test]
fn test_word_serial() {
    let regex = generate_regex("serial");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("সিরিয়াল"));
}

#[test]
fn test_word_development() {
    let regex = generate_regex("development");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ডেভেলপমেন্ট"));
}

#[test]
fn test_word_outline() {
    let regex = generate_regex("outline");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("আউটলাইন"));
}

#[test]
fn test_word_biochemistry() {
    let regex = generate_regex("biochemistry");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("বায়োকেমিস্ট্রি"));
}

#[test]
fn test_word_anatomy() {
    let regex = generate_regex("anatomy");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("অ্যানাটমি"));
}

#[test]
fn test_word_fountain() {
    let regex = generate_regex("fountain");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ফাউন্টেন"));
}

#[test]
fn test_word_logo() {
    let regex = generate_regex("logo");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("লোগো"));
}

#[test]
fn test_word_mexico() {
    let regex = generate_regex("mexico");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("মেক্সিকো"));
}

#[test]
fn test_word_electric() {
    let regex = generate_regex("electric");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ইলেক্ট্রিক"));
}

#[test]
fn test_word_talk() {
    let regex = generate_regex("talk");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("টক"));
}

#[test]
fn test_word_electronics() {
    let regex = generate_regex("electronics");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ইলেক্ট্রনিক্স"));
}

#[test]
fn test_word_settings() {
    let regex = generate_regex("settings");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("সেটিংস"));
}

#[test]
fn test_word_sign() {
    let regex = generate_regex("sign");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("সাইন"));
}

#[test]
fn test_word_programming() {
    let regex = generate_regex("programming");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("প্রোগ্রামিং"));
}

#[test]
fn test_word_dell() {
    let regex = generate_regex("dell");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ডেল"));
}

#[test]
fn test_word_receptionist() {
    let regex = generate_regex("receptionist");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("রিসেপশনিস্ট"));
}

#[test]
fn test_word_mechanical() {
    let regex = generate_regex("mechanical");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("মেকানিক্যাল"));
}

#[test]
fn test_word_subject() {
    let regex = generate_regex("subject");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("সাবজেক্ট"));
}

#[test]
fn test_word_chemistry() {
    let regex = generate_regex("chemistry");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("কেমিস্ট্রি"));
}

#[test]
fn test_word_minus() {
    let regex = generate_regex("minus");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("মাইনাস"));
}

#[test]
fn test_word_bengal() {
    let regex = generate_regex("bengal");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("বেঙ্গল"));
}

#[test]
fn test_word_format() {
    let regex = generate_regex("format");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ফরম্যাট"));
}

#[test]
fn test_word_insurance() {
    let regex = generate_regex("insurance");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ইনসিওরেন্স"));
    assert!(matcher.is_match("ইন্স্যুরেন্স"));
    assert!(matcher.is_match("ইন্সুরেন্স"));
}

#[test]
fn test_word_automatic() {
    let regex = generate_regex("automatic");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("অটোমেটিক"));
}

#[test]
fn test_word_british() {
    let regex = generate_regex("british");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ব্রিটিশ"));
}

#[test]
fn test_word_image() {
    let regex = generate_regex("image");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("ইমেজ"));
}

#[test]
fn test_word_number() {
    let regex = generate_regex("number");
    let matcher = Regex::new(&regex).unwrap();
    assert!(matcher.is_match("নাম্বার"));
}
