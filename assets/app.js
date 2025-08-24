
// Auto-refresh the page every 30 seconds
setInterval(() => {
    window.location.reload();
}, 30000);

// Show copy notification
function showCopyNotification(address) {
    // Remove any existing notification
    const existing = document.querySelector('.copy-notification');
    if (existing) {
        existing.remove();
    }

    // Create notification element
    const notification = document.createElement('div');
    notification.className = 'copy-notification';
    notification.textContent = `Copied: ${address.substring(0, 20)}...`;
    document.body.appendChild(notification);

    // Show notification with animation
    setTimeout(() => {
        notification.classList.add('show');
    }, 10);

    // Hide and remove notification after 3 seconds
    setTimeout(() => {
        notification.classList.remove('show');
        setTimeout(() => {
            if (notification.parentNode) {
                notification.parentNode.removeChild(notification);
            }
        }, 300);
    }, 3000);
}

// Copy text to clipboard
async function copyToClipboard(text) {
    try {
        if (navigator.clipboard && window.isSecureContext) {
            // Modern async clipboard API
            await navigator.clipboard.writeText(text);
            return true;
        } else {
            // Fallback for older browsers or non-HTTPS
            const textArea = document.createElement('textarea');
            textArea.value = text;
            textArea.style.position = 'fixed';
            textArea.style.left = '-999999px';
            textArea.style.top = '-999999px';
            document.body.appendChild(textArea);
            textArea.focus();
            textArea.select();
            const result = document.execCommand('copy');
            textArea.remove();
            return result;
        }
    } catch (err) {
        console.error('Failed to copy text: ', err);
        return false;
    }
}

// Add interactive effects and click-to-copy functionality
document.addEventListener('DOMContentLoaded', () => {
    // Add click-to-copy functionality for addresses
    const addressElements = document.querySelectorAll('.clickable-address');
    addressElements.forEach(element => {
        element.addEventListener('click', async (e) => {
            e.preventDefault();
            const address = element.getAttribute('data-address');

            // Visual feedback
            element.style.transform = 'scale(0.95)';
            setTimeout(() => {
                element.style.transform = '';
            }, 150);

            // Copy to clipboard
            const success = await copyToClipboard(address);
            if (success) {
                showCopyNotification(address);
            } else {
                // Fallback notification for copy failure
                alert(`Copy failed. Address: ${address}`);
            }
        });
    });

    // Update last updated time
    const updateTime = () => {
        const now = new Date();
        const timeString = now.toISOString().slice(0, 19).replace('T', ' ') + ' UTC';
        const lastUpdateElement = document.querySelector('.last-update');
        if (lastUpdateElement) {
            lastUpdateElement.textContent = `Last updated: ${timeString}`;
        }
    };

    // Update time immediately and then every second
    updateTime();
    setInterval(updateTime, 1000);
});
