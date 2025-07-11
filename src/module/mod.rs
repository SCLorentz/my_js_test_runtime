// this is a modified version of: https://github.com/andreubotella/deno-simple-module-loader/blob/main/src/lib.rs

use data_url::DataUrl;
use deno_core::{
    anyhow::{bail, Error}, futures::FutureExt, ModuleSource, ModuleSourceCode, ModuleSpecifier, ModuleType, ModuleLoader, ModuleLoadResponse, resolve_import, RequestedModuleType, ResolutionKind
};

//mod ts;

pub struct SimpleModuleLoader;

impl ModuleLoader for SimpleModuleLoader
{
    fn resolve(
        &self,
        specifier: &str,
        referrer: &str,
        _kind: ResolutionKind,
    ) -> Result<ModuleSpecifier, Error>
    {
        Ok(resolve_import(specifier, referrer)?)
    }

    fn load(
        &self,
        module_specifier: &ModuleSpecifier,
        _maybe_referrer: Option<&ModuleSpecifier>,
        _is_dyn_import: bool,
        requested_module_type: RequestedModuleType,
    ) -> ModuleLoadResponse
    {
        let module_specifier = module_specifier.clone();

        deno_core::ModuleLoadResponse::Async(async move
        {
            let mut redirect_module_url = None;
            let bytes = match module_specifier.scheme()
            {
                "http" | "https" => {
                    let res = reqwest::get(module_specifier.clone()).await?;

                    let res = res.error_for_status()?;
                    // res.url() is the post-redirect URL.
                    if res.url() != &module_specifier {
                        redirect_module_url = Some(res.url().clone());
                    }
                    res.bytes().await?.to_vec()
                }
                "file" => {
                    let path = match module_specifier.to_file_path() {
                        Ok(path) => path,
                        Err(_) => bail!("Invalid file URL."),
                    };
                    tokio::fs::read(path).await?
                }
                "data" => {
                    let url = match DataUrl::process(module_specifier.as_str()) {
                        Ok(url) => url,
                        Err(_) => bail!("Not a valid data URL."),
                    };
                    match url.decode_to_vec() {
                        Ok((bytes, _)) => bytes,
                        Err(_) => bail!("Not a valid data URL."),
                    }
                }
                schema => bail!("Invalid schema {}", schema),
            };

            // TODO: The MIME types should probably be checked.
            let module_type = match requested_module_type
            {
                RequestedModuleType::None => ModuleType::JavaScript,
                RequestedModuleType::Json => ModuleType::Json,
                RequestedModuleType::Other(_) => {
                    unreachable!("Import types other than JSON are not supported")
                }
            };

            if let Some(redirect_module_url) = redirect_module_url {
                Ok(ModuleSource::new_with_redirect(
                    module_type,
                    ModuleSourceCode::Bytes(bytes.into_boxed_slice().into()),
                    &module_specifier,
                    &redirect_module_url,
                    None,
                ))
            } else {
                Ok(ModuleSource::new(
                    module_type,
                    ModuleSourceCode::Bytes(bytes.into_boxed_slice().into()),
                    &module_specifier,
                    None,
                ))
            }
        }
        .boxed_local())
    }
}