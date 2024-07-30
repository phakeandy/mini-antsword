import { message } from "@tauri-apps/api/dialog";

const warning = "Warning!";

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

function isValidUrl(url) {
    try {
        new URL(url);
        return true;
    } catch (err) {
        return false;
    }
}

function stringToBase64(str) {
    // 将字符串转换为base64编码，处理Unicode字符
    return btoa(
        encodeURIComponent(str).replace(
            /%([0-9A-F]{2})/g,
            function (match, p1) {
                // return String.fromCharCode("0x" + p1);
                return String.fromCharCode(parseInt("0x" + p1, 16));
            }
        )
    );
}


function decodeBase64(base64String) {
    try {
        // 使用 atob 函数进行 Base64 解码
        const decodedBytes = atob(base64String);

        // 创建一个文本解码器，指定 UTF-8 编码
        const decoder = new TextDecoder("utf-8");

        // 将解码后的字节转换为 Uint8Array
        const uint8Array = new Uint8Array(decodedBytes.length);
        for (let i = 0; i < decodedBytes.length; i++) {
            uint8Array[i] = decodedBytes.charCodeAt(i);
        }

        // 使用 UTF-8 解码器解码 Uint8Array
        const decodedString = decoder.decode(uint8Array);

        return decodedString;
    } catch (error) {
        console.error(`Error decoding Base64 string: ${error}`);
        throw error;
    }
}


export { isValidUrl, checkUrlAndPassword, stringToBase64, decodeBase64 }