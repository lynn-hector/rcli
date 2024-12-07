use std::fs;
use clap::Parser;
use zxcvbn::zxcvbn;
use tracing_subscriber;
use rcli::{Opts, process_csv, process_genpass,process_encode,process_decode,
        process_text_verify, process_text_sign, SubCommand,Base64SubCommand, 
        TextSubCommand, process_text_generate,HttpSubCommand, process_http_serve};

#[tokio::main]            
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();
    let opts: Opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => {
            let output = if let Some(output) = opts.output{
            output.clone()
        } else {
            format!("output.{}", opts.format)
        };
        process_csv(&opts.input, output, opts.format)?;
        }
        SubCommand::GenPass(opts) => {
            let password = process_genpass(
                opts.length,
                opts.uppercase,
                opts.lowercase,
                opts.number,
                opts.symbol
            )?;
            println!("{}", password);

            //检查密码强度
            let result = zxcvbn(&password, &[]);
            eprintln!("密码强度: {}", result.score());
        }
        SubCommand::Base64(subcmd) => {
            match subcmd {
                Base64SubCommand::Encode(opts) => {
                    let encoded = process_encode(&opts.input, opts.format)?;
                    println!("{}", encoded);
                }
                Base64SubCommand::Decode(opts) => {
                    let decoded = process_decode(&opts.input, opts.format)?;
                    let decoded = String::from_utf8(decoded)?;
                    println!("{}", decoded);
                }
            }
        }
        SubCommand::Text(subcmd) =>  match subcmd {
            TextSubCommand::Sign(opts) => {
                let signed = process_text_sign(&opts.input, &opts.key, opts.format)?;
                println!("{}", signed);
            }
            TextSubCommand::Verify(opts) => {
                let verified = process_text_verify(&opts.input, &opts.key, opts.format, &opts.sig)?;
                println!("{}", verified);
            }
            TextSubCommand::Generate(opts) => {
                let key = process_text_generate(opts.format)?;
                match opts.format {
                    rcli::TextSignFormat::Blake3 => {
                        let name = opts.output.join("blake3.txt");
                        fs::write(name, &key[0])?;
                    }
                    rcli::TextSignFormat::Ed25519 => {
                        let name = opts.output.join("ed25519");
                        fs::write(name.join("sk"), &key[0])?;
                        fs::write(name.join("pk"), &key[1])?;
                    }
                }
            }
        }
        SubCommand::Http(subcmd) => match subcmd{
            HttpSubCommand::Server(opts) => {
                println!("serving at http://0.0.0.0:{}", opts.port);
                process_http_serve(opts.dir, opts.port).await?;
            }
        }
    }

    Ok(())
}

