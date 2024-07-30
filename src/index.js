// @ts-ignore
// import $ from "jquery";


// const previewBlackPagesBtn = document.getElementById("previewBlackPagesBtn");

// // @ts-ignore
// previewBlackPagesBtn.addEventListener("click", function () {
//     fetch('./src/hack.html')
//         .then(response => response.text())
//         .then(data => {
//             // @ts-ignore
//             document.getElementById('response-message').innerHTML = data;
//         })
//         .catch(error => console.error('Error:', error));
// });

// 黑页预览功能
// @ts-ignore
previewBlackPagesBtn.addEventListener("click", function () {

    // 查找 #response-message 中的 iframe 元素
    const existingIframe = document.querySelector('#response-message iframe');

    if (existingIframe) {
        // 如果存在，则移除旧的 iframe
        existingIframe.remove();
        return;
    }

    fetch('./src/hack.html')
        .then(response => response.text())
        .then(data => {
            // 创建一个新的 iframe 元素
            const iframe = document.createElement('iframe');
            iframe.style.width = '100%'; // 设置宽度
            iframe.style.height = '500px'; // 设置高度
            iframe.style.border = 'none'; // 移除边框
            iframe.srcdoc = data; // 设置 srcdoc 属性为获取到的数据

            // 清空 #response-message 的内容
            // @ts-ignore
            document.getElementById('response-message').innerHTML = '';

            // 将 iframe 添加到 #response-message 中
            // @ts-ignore
            document.getElementById('response-message').appendChild(iframe);
        })
        .catch(error => console.error('Error:', error));
});

// 表单持久化
window.onload = function () {
    const url = document.getElementById("url");
    const password = document.getElementById("password");
    const command = document.getElementById("command");

    if (url === null || password === null || command === null) {
        console.error("Element with id 'url' or 'password' not found.");
        return;
    }

    // @ts-ignore
    url.value = localStorage.getItem("url") || "";
    // @ts-ignore
    password.value = localStorage.getItem("password") || "";
    // @ts-ignore
    command.value = localStorage.getItem("command") || "";
};

// 当输入改变时，存储数据到 localStorage
document
    .querySelectorAll('input[type="text"], input[type="password"]')
    .forEach(function (input) {
        input.addEventListener("input", function () {
            localStorage.setItem(this.name, this.value);
        });
    });