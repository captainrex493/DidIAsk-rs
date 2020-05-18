use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use rand::Rng;

#[command]
fn uwu(ctx: &mut Context, msg: &Message) -> CommandResult {
    let _ = msg.channel_id.say(&ctx.http, "`(◡ ω ◡)`");
    Ok(())
}

#[command]
fn owo(ctx: &mut Context, msg: &Message) -> CommandResult {
    let _ = msg.channel_id.say(&ctx.http, "`(ㆁωㆁ)`");
    Ok(())
}

#[command]
fn smile(ctx: &mut Context, msg: &Message) -> CommandResult {
    let s = format!("`{}`", SMILE[rand::thread_rng().gen_range(0, SMILE.len())]);
    let _ = msg.channel_id.say(&ctx.http, s);
    Ok(())
}

#[command]
fn hug(ctx: &mut Context, msg: &Message) -> CommandResult {
    let s = format!("`{}`", HUG[rand::thread_rng().gen_range(0, HUG.len())]);
    let _ = msg.channel_id.say(&ctx.http, s);
    Ok(())
}

#[command]
fn flex(ctx: &mut Context, msg: &Message) -> CommandResult {
    let s = format!("`{}`", FLEX[rand::thread_rng().gen_range(0, FLEX.len())]);
    let _ = msg.channel_id.say(&ctx.http, s);
    Ok(())
}

#[command]
fn animal(ctx: &mut Context, msg: &Message) -> CommandResult {
    let s = format!("`{}`", ANIMAL[rand::thread_rng().gen_range(0, ANIMAL.len())]);
    let _ = msg.channel_id.say(&ctx.http, s);
    Ok(())
}

#[command]
fn surprise(ctx: &mut Context, msg: &Message) -> CommandResult {
    let s = format!("`{}`", SURPRISE[rand::thread_rng().gen_range(0, SURPRISE.len())]);
    let _ = msg.channel_id.say(&ctx.http, s);
    Ok(())
}

#[command]
fn dance(ctx: &mut Context, msg: &Message) -> CommandResult {
    let s = format!("`{}`", DANCE[rand::thread_rng().gen_range(0, DANCE.len())]);
    let _ = msg.channel_id.say(&ctx.http, s);
    Ok(())
}

#[command]
fn shrug(ctx: &mut Context, msg: &Message) -> CommandResult {
    let s = format!("`{}`", SHRUG[rand::thread_rng().gen_range(0, SHRUG.len())]);
    let _ = msg.channel_id.say(&ctx.http, s);
    Ok(())
}

#[command]
fn flip(ctx: &mut Context, msg: &Message) -> CommandResult {
    let s = format!("`{}`", FLIP[rand::thread_rng().gen_range(0, FLIP.len())]);
    let _ = msg.channel_id.say(&ctx.http, s);
    Ok(())
}

#[command]
fn unflip(ctx: &mut Context, msg: &Message) -> CommandResult {
    let s = format!("`{}`", UNFLIP[rand::thread_rng().gen_range(0, UNFLIP.len())]);
    let _ = msg.channel_id.say(&ctx.http, s);
    Ok(())
}

#[command]
fn sus(ctx: &mut Context, msg: &Message) -> CommandResult {
    let s = format!("`{}`", SUS[rand::thread_rng().gen_range(0, SUS.len())]);
    let _ = msg.channel_id.say(&ctx.http, s);
    Ok(())
}

#[command]
fn cri(ctx: &mut Context, msg: &Message) -> CommandResult {
    let s = format!("`{}`", CRI[rand::thread_rng().gen_range(0, CRI.len())]);
    let _ = msg.channel_id.say(&ctx.http, s);
    Ok(())
}

#[command]
fn yike(ctx: &mut Context, msg: &Message) -> CommandResult {
    let s = format!("`{}`", YIKE[rand::thread_rng().gen_range(0, YIKE.len())]);
    let _ = msg.channel_id.say(&ctx.http, s);
    Ok(())
}

#[command]
fn bear(ctx: &mut Context, msg: &Message) -> CommandResult {
    let s = format!("`{}`", BEAR[rand::thread_rng().gen_range(0, BEAR.len())]);
    let _ = msg.channel_id.say(&ctx.http, s);
    Ok(())
}

#[command]
fn fight(ctx: &mut Context, msg: &Message) -> CommandResult {
    let s = format!("`{}`", FIGHT[rand::thread_rng().gen_range(0, FIGHT.len())]);
    let _ = msg.channel_id.say(&ctx.http, s);
    Ok(())
}
const SMILE: [&str; 32] = [
    "(. ❛ ᴗ ❛.)",
    "(・∀・)",
    "◉‿◉",
    "｡◕‿◕｡",
    "(. ❛ ᴗ ❛.)",
    "(θ‿θ)",
    "ʘ‿ʘ",
    "(✷‿✷)",
    "(◔‿◔)",
    "(◕ᴗ◕✿)",
    "(ʘᴗʘ✿)",
    "(人 •͈ᴗ•͈)",
    "(◍•ᴗ•◍)",
    "( ╹▽╹ )",
    "(≧▽≦)",
    "(☆▽☆)",
    "(✯ᴗ✯)",
    "ಡ ͜ ʖ ಡ",
    "(ㆁωㆁ)",
    "<(￣︶￣)>",
    "(*´ω｀*)",
    "( ꈍᴗꈍ)",
    "(✿^‿^)",
    "^________,_^",
    "(◡ ω ◡)",
    "( ´◡‿ゝ◡`,)",
    "(｡•̀ᴗ-)✧",
    "(◠‿◕)",
    "(◠‿・)—☆,",
    "✧◝(⁰▿⁰)◜,✧",
    "(人*´∀｀),｡*ﾟ+",
    "(ﾉ◕ヮ◕)ﾉ*,.✧ ",
];

const HUG: [&str; 32] = [
    "⊂((・▽・))⊃",
    "(づ｡◕‿‿◕｡)づ",
    "༼ つ ◕‿◕ ༽つ",
    "(づ￣ ³￣)づ",
    "(⊃｡•́‿•̀｡)⊃",
    "ʕっ•ᴥ•ʔっ",
    "(o´･_･)っ",
    "(⊃ • ʖ̫ • )⊃",
    "(つ≧▽≦)つ",
    "(つ✧ω✧)つ",
    "(っ.❛ ᴗ ❛.)っ",
   "～(つˆДˆ)つ｡",
    "☆ლ(´ ❥ `ლ)",
    "⊂(•‿•⊂ )*.",
    "✧⊂(´･◡･⊂ )",
    "∘˚˳°⊂(･ω･*⊂)",
    "⊂(・﹏・⊂)",
    "⊂(・▽・⊂)",
    "⊂(◉‿◉)つ",
    "o((*^▽^*))o",
    "╰(*´︶`*)╯",
    "╰(＾3＾)╯",
    "╰(⸝⸝⸝´꒳`⸝⸝⸝)╯",
    "♡(˃͈ દ ˂͈ ༶ )",
    "ヾ(˙❥˙)ﾉ",
    "＼(^o^)／",
    "ლ(・﹏・ლ)",
    "ლ(◕ω◕ლ)",
    "(/･ω･(-ω-)",
    "(･ω･)つ⊂(･ω･)",
    "( T_T)＼(^-^ )",
    "(･–･) \\(･◡･)/",
];

const FLEX: [&str; 32] = [
    "ᕙ(⇀‸↼‶)ᕗ",
    "ᕙ(＠°▽°＠)ᕗ",
    "ᕙ(  • ‿ •  )ᕗ",
    "୧(﹒︠ᴗ﹒︡)୨",
    "ᕙ (° ~͜ʖ~ °) ᕗ",
    "ᕙ( ͡◉ ͜ ʖ ͡◉)ᕗ",
    "୧(＾ 〰 ＾)୨",
    "ᕙ( ¤ 〰 ¤ )ᕗ",
    "ᕙ( ~ . ~ )ᕗ",
    "ᕙ( : ˘ ∧ ˘ : )ᕗ",
    "ᕙ[･۝･]ᕗ",
    "ᕦ( ⊡ 益 ⊡ )ᕤ",
    "ᕙ(ಠ ਊ ಠ)ᕗ",
    "ᕙ(☉ਊ☉)ᕗ",
    "୧( ಠ Д ಠ )୨",
    "ᕦ⊙෴⊙ᕤ",
    "ᕦ(ò_óˇ)ᕤ",
    "ᕦ(ಠ_ಠ)ᕤ",
    "ᕦ[ ◑ □ ◑ ]ᕤ",
    "ᕦ༼ ~ •́ ₒ •̀ ~ ༽ᕤ",
    "୧( ˵ ° ~ ° ˵ )୨",
    "୧| ͡ᵔ ﹏ ͡ᵔ |୨",
    "ᕙ (° ~ ° ~)",
    "( ͝° ͜ʖ͡°)ᕤ",
    "ᕙ( ͡° ͜ʖ ͡°)ᕗ",
    "ᕙ(͡°‿ ͡°)ᕗ",
    "ᕦ༼ຈل͜ຈ༽ᕤ",
    "༼ᕗຈل͜ຈ༽ᕗ",
    "ᕙ༼◕ ᴥ ◕༽ᕗ",
    "ᕦʕ •ᴥ•ʔᕤ",
    "ᕦᶘ ᵒ㉨ᵒᶅᕤ",
    "ᕦ༼✩ل͜✩༽ᕤ ",
];

const ANIMAL: [&str; 32] = [
    "V●ᴥ●V",
    "▼・ᴥ・▼",
    "U ´꓃ ` U",
    "(◠ᴥ◕ʋ)",
    "U^ｪ^U ",
    "( ͡°ᴥ ͡° ʋ)",
    "◖⚆ᴥ⚆◗",
    "/ᐠ｡ꞈ｡ᐟ\\",
    "ฅ^•ﻌ•^ฅ",
    "(=^･ｪ･^=)",
    "(=｀ェ´=)",
    "(￣(ｴ)￣)ﾉ",
    "(*￣(ｴ)￣*)",
    "(≧(ｴ)≦ )",
    "(´(ｪ)｀）",
    "(・(ｪ)・）",
    "(✪㉨✪)",
    "ʕ·ᴥ·ʔ",
    "ʕ ꈍᴥꈍʔ",
    "ʕ´•ᴥ•`ʔ",
    "(✪㉨✪)",
    "(◕ᴥ◕)",
    "(ᵔᴥᵔ)",
    "Ꮚ˘ ꈊ ˘ Ꮚ",
    "(´・(oo)・｀)",
    "(^._.^)ﾉ",
    "～>`)～～～",
    "…ᘛ⁐̤ᕐᐷ",
    "くコ:彡",
    "-ᄒᴥᄒ-",
    "/╲/\\╭(•‿•)╮/\\╱\\",
    "Ƹ̵̡Ӝ̵̨̄Ʒ",
];

const SURPRISE: [&str; 32] = [
    "(･o･;)",
    "(・o・)",
    "(゜o゜;",
    "w(°ｏ°)w",
    "(☉｡☉)!",
    "(@_@)",
    "ヽ((◎д◎))ゝ",
    "＼(°o°)／",
    "ヽ(｡◕o◕｡)ﾉ.",
    "＼(◎o◎)／",
    "ヾ(*’Ｏ’*)/",
    "✧\\(>o<)ﾉ✧",
    "(ﾉ*0*)ﾉ",
    "ヽ༼⁰o⁰；༽ノ",
    "⋋✿ ⁰ o ⁰ ✿⋌",
    "щ(゜ロ゜щ)",
    "(ﾉﾟ0ﾟ)ﾉ",
    "~ლ(^o^ლ)",
    "(ﾟοﾟ人))",
    "⊙.☉",
    "(⑉⊙ȏ⊙)",
    "(ʘᗩʘ’)",
    "(‘◉⌓◉’)",
    "⁄(⁄ ⁄•⁄-⁄•⁄ ⁄)⁄",
    "(｡☬０☬｡)",
    "(´⊙ω⊙`)！",
    "(((;ꏿ_ꏿ;)))",
    "(●__●)",
    "(✿☉｡☉)",
    "(>0<；)",
    "༼⁰o⁰；༽",
    "(╬⁽⁽ ⁰ ⁾⁾ Д ⁽⁽ ⁰ ⁾⁾)",
];

const DANCE: [&str; 33] = [
    "♪~ ᕕ(ᐛ)ᕗ",
    "♪～(´ε｀ )",
    "(＾3＾♪",
    "┌(・。・)┘♪",
    "♪ヽ(･ˇ∀ˇ･ゞ)",
    "⁽⁽◝( •௰• )◜⁾⁾",
    "₍₍◞( •௰• )◟₎₎",
    "⁽⁽ଘ( ˊᵕˋ )ଓ⁾⁾",
    "♪ \\(^ω^\\ )",
    "\\(ϋ)/♩",
    "♪┌|∵|┘♪ ",
    "└|∵|┐♪",
    "♪ \\(^ω^\\ )",
    "( /^ω^)/♪",
    "♪(＾∇＾)ﾉ♪",
    "ヾ( ͝° ͜ʖ͡°)ノ♪",
    "\\(๑╹◡╹๑)ﾉ♬",
    "(*ﾉ・ω・)ﾉ♫",
    "┌|o^▽^o|┘♪",
    "┏(＾0＾)┛",
    "┌(★ｏ☆)┘",
    "└( ＾ω＾)」",
    "(｢`･ω･)｢",
    "♪(┌・。・)┌",
    "ヘ(￣ω￣ヘ)",
    "ƪ(‾.‾“)┐",
    "ƪ(˘⌣˘)ʃ",
    "(ノ^_^)ノ",
    "＼(ﾟｰﾟ＼)",
    "ヽ(*ﾟｰﾟ*)ﾉ",
    "ヾ(･ω･*)",
    "(~‾▿‾)~",
    "〜(꒪꒳꒪)〜",
];

const SHRUG: [&str; 32] = [
    "¯\\_(ツ)_/¯",
    "¯\\_༼ •́ ͜ʖ •̀ ༽_/¯",
    "¯\\_( ͡° ͜ʖ ͡°)_/¯",
    "¯\\(°_o)/¯",
    "┐( ∵ )┌",
    "¯\\_༼ᴼل͜ᴼ༽_/¯",
    "¯\\_༼ ಥ ‿ ಥ ༽_/¯",
    "乁༼☯‿☯✿༽ㄏ",
    "¯\\(◉‿◉)/¯",
    "¯\\_ʘ‿ʘ_/¯",
    "¯\\_༼ ಥ ‿ ಥ ༽_/¯",
    "╮(＾▽＾)╭",
    "乁[ ◕ ᴥ ◕ ]ㄏ",
    "乁[ᓀ˵▾˵ᓂ]ㄏ",
    "┐(´(エ)｀)┌",
    "┐( ˘_˘)┌",
    "┐(´ー｀)┌",
    "╮(╯_╰)╭",
    "¯\\_(⊙_ʖ⊙)_/¯",
    "乁( ⁰͡ Ĺ̯ ⁰͡ ) ㄏ",
    "¯\\_( ͠° ͟ʖ °͠ )_/¯",
    "乁( •_• )ㄏ",
    "乁| ･ 〰 ･ |ㄏ",
    "┐(‘～`;)┌",
    "┐(￣ヘ￣)┌",
    "┐(´д`)┌",
    "乁( . ര ʖ̯ ര . )ㄏ",
    "乁 ˘ o ˘ ㄏ",
    "乁ʕ •̀ ۝ •́ ʔㄏ",
    "¯\\_〳 •̀ o •́ 〵_/¯",
    "¯\\_(☯෴☯)_/¯",
    "乁║ ˙ 益 ˙ ║ㄏ",
];

const FLIP: [&str; 26] = [
    "(╯°□°）╯︵ ┻━┻",
    "(ノ｀Д´)ノ彡┻━┻",
    "(┛◉Д◉)┛彡┻━┻",
    "(ﾉ≧∇≦)ﾉ ﾐ ┻━┻",
    "(ノಠ益ಠ)ノ彡┻━┻",
    "(╯ರ ~ ರ)╯︵ ┻━┻",
    "(┛ಸ_ಸ)┛彡┻━┻",
    "(ﾉ´･ω･)ﾉ ﾐ ┻━┻",
    "(ノಥ,_｣ಥ)ノ彡┻━┻",
    "(┛✧Д✧))┛彡┻━┻",
    "┻┻︵¯\\(ツ)/¯︵┻┻",
    "┻┻︵ヽ(`Д´)ﾉ︵┻┻",
    "(/¯◡ ‿ ◡)/¯ ~ ┻━┻",
    "(ノ｀⌒´)ノ┫：・┻┻",
    "(ﾉ°_o)ﾉ⌒┫ ┻ ┣ ┳",
    "┻━┻ミ＼(≧ﾛ≦＼)",
    "┻━┻︵└(՞▽՞ └)",
    "┻━┻︵└(´_｀└)",
    "┻━┻ ヘ╰( •̀ε•́ ╰)",
    "─=≡Σ(╯°□°)╯︵┻┻",
    "(ノ•̀ o •́ )ノ ~ ┻━┻",
    "(-_- )ﾉ⌒┫ ┻ ┣ ┳",
    "(ノ￣皿￣)ノ ⌒== ┫",
    "(┛❍ᴥ❍)┛彡┻━┻",
    "(ノT＿T)ノ ＾┻━┻",
    "ʕノ•ᴥ•ʔノ ︵ ┻━┻",
];

const UNFLIP: [&str; 6] = [
    "┬─┬ノ( ͡° ͜ʖ ͡°ノ)",
    "┬─┬ノ(ಠ_ಠノ)",
    "┬─┬ノ( º _ ºノ)",
    "┬──┬◡ﾉ(° -°ﾉ",
    "(ヘ･_･)ヘ┳━┳",
    "┬──┬ ¯\\_(ツ)",
];

const SUS: [&str; 33] = [
    "ಠ_ಠ",
    "ಠ_ʖಠ",
    "ಠ︵ಠ",
    "ಠ ೧ ಠ",
    "ಠಗಠ",
    "ಠ,_｣ಠ",
    "ಠωಠ",
    "ಠ ͜ʖ ಠ",
    "ಠ◡ಠ",
    "ಠ∀ಠ",
    "ಠ﹏ಠ",
    "ಠ‿ಠ",
    "ಠ益ಠ",
    "ಠᴥಠ",
    "ʕಠ_ಠʔ",
    "Σ(ಠ_ಠ)",
    "(ಠ_ಠ)",
    ">⌐■-■",
    "(⌐■-■)",
    "[̲̅$̲̅(̲̅ ͡ಠ_ಠ)̲̅$̲̅]",
    "ಠ ل͟ ಠ",
    "(ノಠ益ಠ)ノ",
    "(ಠ_ಠ)━☆ﾟ.*･｡ﾟ",
    "¯\\_ಠ_ಠ_/¯",
    "ರ_ರ",
    "ರ╭╮ರ",
    "(눈‸눈)",
    "(ب_ب)",
    "ತ_ತ",
    "ತ_ʖತ",
    "ಠಿ_ಠಿ",
    "ಠಿ_ಠ",
    "ಠಿヮಠ",
];

const CRI: [&str; 33] = [
    "(;´༎ຶД༎ຶ`)",
    "•́  ‿ ,•̀ ",
    "ಥ‿ಥ",
    "ʕ´• ᴥ•̥`ʔ",
    "༎ຶ‿༎ຶ",
    "( ；∀；)",
    "(´；ω；｀)",
    "(･ัω･ั)",
    "(╯︵╰,)",
    "Ó╭╮Ò",
    "(っ˘̩╭╮˘̩)っ",
    "( ･ั﹏･ั)",
    "(｡ŏ﹏ŏ)",
    "(๑´•.̫ • `๑)",
    "(´ . .̫ . `)",
    "(｡•́︿•̀｡)",
    "(｡ﾉω＼｡)",
    "ಥ╭╮ಥ",
    "(ᗒᗩᗕ)",
    "( ≧Д≦)",
    ".·´¯`(>▂<)´¯`·.",
    "( ⚈̥̥̥̥̥́⌢⚈̥̥̥̥̥̀)",
    "ಥ_ಥ",
    "(´;︵;`)",
    "༼;´༎ຶ ۝ ༎ຶ༽",
    "｡:ﾟ(;´∩`;)ﾟ:｡",
    "(༎ຶ ෴ ༎ຶ)",
    "( ꈨຶ ˙̫̮ ꈨຶ )",
    "(〒﹏〒)",
    "(个_个)",
    "(╥﹏╥)",
    "(-̩̩̩-̩̩̩-̩̩̩-̩̩̩-̩̩̩___-̩̩̩-̩̩̩-̩̩̩-̩̩̩-̩̩̩)",
    "(´°̥̥̥̥̥̥̥̥ω°̥̥̥̥̥̥̥̥｀)",
];

const YIKE: [&str; 32] = [
    "(๑•﹏•)",
    "⊙﹏⊙",
    "╏ ” ⊚ ͟ʖ ⊚ ” ╏",
    "(╬☉д⊙)⊰⊹ฺ",
    "ヘ（。□°）ヘ",
    "(⊙_◎)",
    "ミ●﹏☉ミ",
    "(●´⌓`●)",
    "(*﹏*;)",
    "(＠_＠;)",
    "(ꏿ﹏ꏿ;)",
    "(;ŏ﹏ŏ)",
    "(• ▽ •;)",
    "(˘･_･˘)",
    "(*・～・*)",
    "(・_・;)",
    "(;;;・_・)",
    "(・–・;)",
    "ゞ(^～^;)ゞ",
    "(￣ヘ￣;)",
    "(٥↼_↼)",
    "(ー_ー゛)",
    "(─.─||）",
    "(-_-;)",
    "(-_-メ)",
    "(-_-;)",
    "・・・(´-﹏-`；)",
    "(~_~メ)",
    "(~_~;)",
    "(ʘ言ʘ╬)",
    "(^_^メ)",
    "(；^ω^）",
];

const BEAR: [&str; 10] = [
    "ʕっ•ᴥ•ʔっ",
    "ᕦʕ •ᴥ•ʔᕤ",
    "ʕ º ᴥ ºʔ",
    "ʕ·ᴥ·ʔ",
    "ʕ ꈍᴥꈍʔ",
    "ʕ´•ᴥ•`ʔ",
    "乁ʕ •̀ ۝ •́ ʔㄏ",
    "ʕಠ_ಠʔ",
    "ʕ´• ᴥ•̥`ʔ",
    "ʕノ•ᴥ•ʔノ ︵ ┻━┻",
];

const FIGHT: [&str; 2] = [
    "(ง'̀-'́)ง",
    "(ง ͠° ͟ل͜ ͡°)ง",
];