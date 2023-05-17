() => {
    const mime = 'image/jpg';
    const bstr = atob(`{image_base64}`);
    let n = bstr.length;
    let u8arr = new Uint8Array(n);

    while (n--) {
        u8arr[n] = bstr.charCodeAt(n);
    }

    const fileInput = document.createElement('input');
    fileInput.type = 'file';
    fileInput.name = 'avatar';

    const file = new File([u8arr], 'minecraft.jpg', {type: mime});

    const dataTransfer = new DataTransfer();
    dataTransfer.items.add(file);
    fileInput.files = dataTransfer.files;


    const form = document.querySelector('form');
    form.appendChild(fileInput);
    form.submit();
}