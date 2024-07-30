import { invoke } from "@tauri-apps/api/tauri";
import { checkUrlAndPassword } from "../modules/utils";
import $ from "jquery";


async function echoCommandResponse(url, password, command) {
    checkUrlAndPassword(url, password);

    try {
        const response = await invoke("base64_encode_system_command", { url, key: password, command });

        // base64 decode
        const decoder = new TextDecoder("utf-8");
        const decodedText = decoder.decode(new Uint8Array(atob(response).split('').map(c => c.charCodeAt(0))));

        const htmlSafeText = decodedText.replace(/\n/g, '<br>');
        $("#response-message").html(htmlSafeText);
    } catch (e) {
        $("#response-message").text(
            "An error occurred while echoing the command response."
        );
        console.error("Error in :", e);
    }
}

export { echoCommandResponse };