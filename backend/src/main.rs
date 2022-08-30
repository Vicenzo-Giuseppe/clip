use moon::*;

async fn frontend() -> Frontend {
    Frontend::new().title("joji in Yukon").append_to_head(
        "
        <style>
            html {
                background-color: #232323;
                color: #643FFF;
            }
        </style>",
    )
}

async fn up_msg_handler(_: UpMsgRequest<()>) {}

#[moon::main]
async fn main() -> std::io::Result<()> {
    start(frontend, up_msg_handler, |_| {}).await
}
