window.triggerFileInput = function() {
    const input = document.createElement('input');
    input.type = 'file';
    input.accept = 'image/*';
    input.onchange = function(e) {
        const file = e.target.files[0];
        if (file) {
            const reader = new FileReader();
            reader.onload = function(event) {
                // Send data back to Rust
                if (window.rustFileCallback) {
                    window.rustFileCallback(event.target.result, file.name);
                }
            };
            reader.readAsDataURL(file);
        }
    };
    input.click();
};
