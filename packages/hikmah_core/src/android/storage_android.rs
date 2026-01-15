use std::path::PathBuf;

pub struct StorageManager;

impl StorageManager {
    pub fn get_storage_path() -> Result<PathBuf, String> {
        Self::get_android_files_dir()
            .map(|p| PathBuf::from(p).join("ebooks"))
            .map_err(|e| format!("Failed to get Android storage: {}", e))
    }

    pub fn get_library_path() -> Result<PathBuf, String> {
        Self::get_android_files_dir()
            .map(|p| PathBuf::from(p).join("library.json"))
            .map_err(|e| format!("Failed to get Android storage: {}", e))
    }

    fn get_android_files_dir() -> Result<String, String> {
        let ctx = ndk_context::android_context();
        let vm = unsafe { jni::JavaVM::from_raw(ctx.vm().cast()) }
            .map_err(|e| format!("JNI error: {}", e))?;
        let mut env = vm
            .attach_current_thread()
            .map_err(|e| format!("Failed to attach thread: {}", e))?;
        let ctx = unsafe { jni::objects::JObject::from_raw(ctx.context().cast()) };

        let files_dir = env
            .call_method(ctx, "getFilesDir", "()Ljava/io/File;", &[])
            .map_err(|e| format!("Failed to call getFilesDir: {}", e))?
            .l()
            .map_err(|e| format!("Failed to get object: {}", e))?;

        let path_str: jni::objects::JString = env
            .call_method(&files_dir, "getAbsolutePath", "()Ljava/lang/String;", &[])
            .map_err(|e| format!("Failed to get path: {}", e))?
            .l()
            .map_err(|e| format!("Failed to get string: {}", e))?
            .try_into()
            .map_err(|e| format!("Failed to convert: {}", e))?;

        let path = env
            .get_string(&path_str)
            .map_err(|e| format!("Failed to get string: {}", e))?;
        Ok(path
            .to_str()
            .map_err(|e| format!("Invalid UTF-8: {}", e))?
            .to_string())
    }

    pub fn get_default_browse_path() -> PathBuf {
        if let Ok(path) = std::env::var("EXTERNAL_STORAGE") {
            PathBuf::from(path).join("Download")
        } else {
            PathBuf::from("/storage/emulated/0/Download")
        }
    }
}
