window.openGallery = function(inputId) {
    const input = document.getElementById(inputId);
    if (input) {
        input.click();
    }
};

window.handleFileSelect = function(inputId, callback) {
    const input = document.getElementById(inputId);
    if (input && input.files && input.files[0]) {
        const file = input.files[0];
        const reader = new FileReader();
        reader.onload = function(e) {
            callback(e.target.result, file.name);
        };
        reader.readAsDataURL(file);
    }
};
