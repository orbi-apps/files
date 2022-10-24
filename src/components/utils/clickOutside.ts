export function clickOutside(node, { enabled: initialEnabled, cb }) {
    const handleOutsideClick = ({ target }) => {
        if (!node.contains(target)) {
            cb();
        }
    };

    function update({enabled}) {
        setTimeout(() => {
            if (enabled) {
                window.addEventListener('click', handleOutsideClick);
            } else {
                window.removeEventListener('click', handleOutsideClick);
            }
        }, 200)
    }

    update({ enabled: initialEnabled });
    return {
        update,
        destroy() {
            window.removeEventListener( 'click', handleOutsideClick );
        }
    };
}