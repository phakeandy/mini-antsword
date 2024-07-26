import { invoke } from "@tauri-apps/api/tauri";

import { ask } from "@tauri-apps/api/dialog";
import { message } from "@tauri-apps/api/dialog";

// const yes = await ask("Are you sure?", "Tauri");

const warning = "Warning!";

async function test() {
  try {
    const message = await invoke("my_custom_command");
    document.getElementById("message").innerText = message;
  } catch (e) {
    console.error("Error invoking command:", e);
  }
}

async function checkUrlAndPassword(url, password) {
  if (password == null || url == null) {
    await message("URL和密码不能为空", warning);
    return;
  } // 防止为空

  if (!isValidUrl(url)) {
    // alert("Not a valid url!");
    await message("不合法的 URL", warning);
    console.error("Not a valid url!");
    return;
  }
}

async function passUrl(url, password) {

  checkUrlAndPassword(url, password);

  try {
    const response = await invoke("submit_form", { url, password });
    $("#respons-message").text(response);
  } catch (e) {
    console.error("Error submitting form:", e);
    $("#response-message").text(
      "An error occurred while submitting the form."
    );
  }

}

function isValidUrl(url) {
  try {
    new URL(url);
    return true;
  } catch (err) {
    return false;
  }
}

async function echoCommandResponse(url, password, command) {
  checkUrlAndPassword(url, password);

  try {
    const response = await invoke("base64_encode_system_command", { url, key: password, command });

    // base64 decode
    const decoder = new TextDecoder("utf-8");
    const decodedText = decoder.decode(new Uint8Array(atob(response).split('').map(c => c.charCodeAt(0))));

    const htmlSafeText = decodedText.replace(/\n/g, '<br>');
    $("#respons-message").html(htmlSafeText);
  } catch (e) {
    console.error("Error in :", e);
    $("#response-message").text(
      "An error occurred while ..."
    );
  }
}



// jquery 写法
$(document).ready(function () {
  $("#invoke-button").click(test);
});

$(document).ready(function () {
  $("#connectToTarget").on("submit", async function (event) {
    event.preventDefault();
    const url = $("#url").val();
    const password = $("#password").val();
    // passUrl(url, password);

    const command = $("#command").val();

    echoCommandResponse(url, password, command);
  });
});

