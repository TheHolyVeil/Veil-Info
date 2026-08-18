slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    // Content lives here. Edit + recompile to update.
    // Placeholder copy below — swap in your actual reasoning/notes.
    let pages: Vec<DocEntry> = vec![
        DocEntry {
            title: "What & Why".into(),
            body: "Veil is a full Wayland compositor — not the v1 terminal- \
display gizmo it started as. Single Rust codebase, no Zig. Ships with a \
built-in app launcher and terminal, plus a custom render engine sitting \
on top of the Wayland stack. Idles around 70MB RAM, which is absurdly \
low for the amount of custom rendering it's doing."
                .into(),
        },
        DocEntry {
            title: "Architecture".into(),
            body: "veil-compositor (Rust) owns the Wayland protocol surface, \
window management, and input.\n\n\
veil-render (Rust) is the custom render engine: WGSL shaders for the \
normal rendering path, plus an ASCII render mode and SSH support for \
remote/low-bandwidth sessions.\n\n\
veil-config (Rust, mlua) exposes compositor behavior to Lua for keybinds \
and layout config.\n\n\
There is no veil-cli — that piece was dropped.\n\n\
WGSL over raw GLSL or wgpu's defaults: Veil has to run everywhere, from a \
0.3GHz single-core i686 box to a maxed-out desktop, so precise control \
over what gets rendered and how is critical for managing resources across \
that whole range. GLSL alone was too raw — too little structure to manage \
that predictably. WGSL sits at the point of \"controlled but not rigid,\" \
which is exactly what that spread of hardware demands.\n\n\
The ASCII render mode exists for machines that can't hit a usable \
framerate any other way — no GPU, or just not enough of one, where a \
GPU-accurate path would look worse than ASCII done well. It wasn't built \
for SSH originally. Once it turned out ASCII rendering could carry a full \
compositor session over SSH — not just TUI/CLI, but running a real web \
browser through it — that became a capability in its own right, not just \
a fallback.".into(),
        },
        DocEntry {
            title: "Philosophy".into(),
            body: "The RAM number (compositor idles ~70MB, full stack — \
executable plus everything the render engine needs — lands around 100MB) \
reads as bloated until you see the tradeoff being made. Vulkan and OpenGL \
were both ruled out: too bloated, and they don't reliably run across \
Veil's actual target range (0.3GHz single-core i686 through top-end \
desktops). Instead everything the render pipeline needs is compiled \
straight into the binary — near-zero external dependencies, runs almost \
anywhere as a result.\n\n\
The corner cut on purpose: no aggressive RAM auto-allocation, minimal \
CPU-side optimization work. That's a deliberate trade, not neglect — bias \
hard toward RAM to keep CPU usage near zero, because CPU headroom is what \
low-power machines don't have. The executable plus ~100MB RAM is the \
entire budget for running Veil.".into(),
        },
        DocEntry {
            title: "VeilLogin".into(),
            body: "VeilLogin is Veil's login manager — Slint-based, TWM/sage \
palette, its own greeter rather than reusing an existing one. Idles around \
20MB RAM with a moving image background and animations.\n\n\
The moving image is simpler than it looks: it's loaded with the `image` \
crate, then compressed in RAM, with overlay effects layered on top. \
Compression is the actual trick here, not the animation itself — that's \
where the RAM stays low.\n\n\
Slint is kept mostly separate from the Rust side, so once login succeeds \
the Slint UI just evaporates and the TTY is freed up / swapped to desktop. \
VeilLogin keeps a daemon running in the background after that point.\n\n\
There's no handoff to veil-compositor — that was tried and dropped. \
Parsing per-user folders to locate and launch the right veil binary broke \
userspace boundaries in ways that weren't worth it, so VeilLogin and Veil \
stay decoupled.".into(),
        },
    ];

    let ui = AppWindow::new()?;
    ui.set_pages(std::rc::Rc::new(slint::VecModel::from(pages)).into());
    ui.run()
}
