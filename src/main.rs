use enigo::{Enigo, MouseButton, MouseControllable};
use hotkey;

const VK_F6: u32 = 117;
const VK_F7: u32 = 118;

fn main() {
    println!(
        "
    右键hotkey.exe,管理员权限打开
    F6:按下鼠标左右键
    F7:释放鼠标左右键
    "
    );
    let mut hk = hotkey::Listener::new();

    hk.register_hotkey(0 | 0, VK_F6, move || {
        let mut enigo = Enigo::new();
        enigo.mouse_down(MouseButton::Right);
        enigo.mouse_down(MouseButton::Left);
        println!("已经按下鼠标左右键");
        // enigo.mouse_move_to(0, 0);
        drop(enigo)
    })
    .unwrap();

    hk.register_hotkey(0 | 0, VK_F7, || {
        let mut enigo = Enigo::new();
        enigo.mouse_move_to(960, 540);
        enigo.mouse_up(MouseButton::Right);
        enigo.mouse_up(MouseButton::Left);
        println!("已经释放鼠标左右键");
        // drop(enigo)
    })
    .unwrap();

    hk.listen();
}
