import { invoke } from "@tauri-apps/api/tauri";
import { message } from "@tauri-apps/api/dialog";
import { ERROR } from "../constent";
import { stringToBase64 } from "./utils";

async function upload_file(url, password, fileContent, filename) {
    try {
        const base64Content = stringToBase64(fileContent);
        console.log(base64Content);
        await invoke("upload_file", { fileBase64Content: base64Content, url: url, key: password, filename: filename })
        await message("Upload Success", "Success");
    } catch (error) {
        await message("Error: " + error, ERROR);
        console.log("Error in uploading file: " + error);
    }
}

export { upload_file }  