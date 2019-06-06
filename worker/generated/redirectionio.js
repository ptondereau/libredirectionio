(function() {
    const __exports = {};
    let wasm;

    /**
    * @returns {void}
    */
    __exports.redirectionio_init_log = function() {
        return wasm.redirectionio_init_log();
    };

    let WASM_VECTOR_LEN = 0;

    let cachedTextEncoder = new TextEncoder('utf-8');

    let cachegetUint8Memory = null;
    function getUint8Memory() {
        if (cachegetUint8Memory === null || cachegetUint8Memory.buffer !== wasm.memory.buffer) {
            cachegetUint8Memory = new Uint8Array(wasm.memory.buffer);
        }
        return cachegetUint8Memory;
    }

    let passStringToWasm;
    if (typeof cachedTextEncoder.encodeInto === 'function') {
        passStringToWasm = function(arg) {

            let size = arg.length;
            let ptr = wasm.__wbindgen_malloc(size);
            let writeOffset = 0;
            while (true) {
                const view = getUint8Memory().subarray(ptr + writeOffset, ptr + size);
                const { read, written } = cachedTextEncoder.encodeInto(arg, view);
                writeOffset += written;
                if (read === arg.length) {
                    break;
                }
                arg = arg.substring(read);
                ptr = wasm.__wbindgen_realloc(ptr, size, size += arg.length * 3);
            }
            WASM_VECTOR_LEN = writeOffset;
            return ptr;
        };
    } else {
        passStringToWasm = function(arg) {

            const buf = cachedTextEncoder.encode(arg);
            const ptr = wasm.__wbindgen_malloc(buf.length);
            getUint8Memory().set(buf, ptr);
            WASM_VECTOR_LEN = buf.length;
            return ptr;
        };
    }

    let cachedTextDecoder = new TextDecoder('utf-8');

    function getStringFromWasm(ptr, len) {
        return cachedTextDecoder.decode(getUint8Memory().subarray(ptr, ptr + len));
    }

    let cachedGlobalArgumentPtr = null;
    function globalArgumentPtr() {
        if (cachedGlobalArgumentPtr === null) {
            cachedGlobalArgumentPtr = wasm.__wbindgen_global_argument_ptr();
        }
        return cachedGlobalArgumentPtr;
    }

    let cachegetUint32Memory = null;
    function getUint32Memory() {
        if (cachegetUint32Memory === null || cachegetUint32Memory.buffer !== wasm.memory.buffer) {
            cachegetUint32Memory = new Uint32Array(wasm.memory.buffer);
        }
        return cachegetUint32Memory;
    }
    /**
    * @param {string} project_id
    * @param {string} rules_data
    * @param {boolean} cache
    * @returns {string}
    */
    __exports.update_rules_for_router = function(project_id, rules_data, cache) {
        const ptr0 = passStringToWasm(project_id);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm(rules_data);
        const len1 = WASM_VECTOR_LEN;
        const retptr = globalArgumentPtr();
        wasm.update_rules_for_router(retptr, ptr0, len0, ptr1, len1, cache);
        const mem = getUint32Memory();
        const rustptr = mem[retptr / 4];
        const rustlen = mem[retptr / 4 + 1];

        const realRet = getStringFromWasm(rustptr, rustlen).slice();
        wasm.__wbindgen_free(rustptr, rustlen * 1);
        return realRet;

    };

    /**
    * @param {string} project_id
    * @param {string} url
    * @returns {string}
    */
    __exports.get_rule_for_url = function(project_id, url) {
        const ptr0 = passStringToWasm(project_id);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm(url);
        const len1 = WASM_VECTOR_LEN;
        const retptr = globalArgumentPtr();
        wasm.get_rule_for_url(retptr, ptr0, len0, ptr1, len1);
        const mem = getUint32Memory();
        const rustptr = mem[retptr / 4];
        const rustlen = mem[retptr / 4 + 1];
        if (rustptr === 0) return;
        const realRet = getStringFromWasm(rustptr, rustlen).slice();
        wasm.__wbindgen_free(rustptr, rustlen * 1);
        return realRet;

    };

    /**
    * @param {string} project_id
    * @param {string} url
    * @returns {string}
    */
    __exports.get_trace_for_url = function(project_id, url) {
        const ptr0 = passStringToWasm(project_id);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm(url);
        const len1 = WASM_VECTOR_LEN;
        const retptr = globalArgumentPtr();
        wasm.get_trace_for_url(retptr, ptr0, len0, ptr1, len1);
        const mem = getUint32Memory();
        const rustptr = mem[retptr / 4];
        const rustlen = mem[retptr / 4 + 1];
        if (rustptr === 0) return;
        const realRet = getStringFromWasm(rustptr, rustlen).slice();
        wasm.__wbindgen_free(rustptr, rustlen * 1);
        return realRet;

    };

    /**
    * @param {string} rule_str
    * @param {string} url
    * @param {number} response_code
    * @returns {string}
    */
    __exports.get_redirect = function(rule_str, url, response_code) {
        const ptr0 = passStringToWasm(rule_str);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm(url);
        const len1 = WASM_VECTOR_LEN;
        const retptr = globalArgumentPtr();
        wasm.get_redirect(retptr, ptr0, len0, ptr1, len1, response_code);
        const mem = getUint32Memory();
        const rustptr = mem[retptr / 4];
        const rustlen = mem[retptr / 4 + 1];
        if (rustptr === 0) return;
        const realRet = getStringFromWasm(rustptr, rustlen).slice();
        wasm.__wbindgen_free(rustptr, rustlen * 1);
        return realRet;

    };

    /**
    * @param {string} rule_str
    * @param {string} headers_str
    * @returns {string}
    */
    __exports.header_filter = function(rule_str, headers_str) {
        const ptr0 = passStringToWasm(rule_str);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm(headers_str);
        const len1 = WASM_VECTOR_LEN;
        const retptr = globalArgumentPtr();
        wasm.header_filter(retptr, ptr0, len0, ptr1, len1);
        const mem = getUint32Memory();
        const rustptr = mem[retptr / 4];
        const rustlen = mem[retptr / 4 + 1];

        const realRet = getStringFromWasm(rustptr, rustlen).slice();
        wasm.__wbindgen_free(rustptr, rustlen * 1);
        return realRet;

    };

    /**
    * @param {string} rule_str
    * @param {string} filter_id
    * @returns {string}
    */
    __exports.create_body_filter = function(rule_str, filter_id) {
        const ptr0 = passStringToWasm(rule_str);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm(filter_id);
        const len1 = WASM_VECTOR_LEN;
        const retptr = globalArgumentPtr();
        wasm.create_body_filter(retptr, ptr0, len0, ptr1, len1);
        const mem = getUint32Memory();
        const rustptr = mem[retptr / 4];
        const rustlen = mem[retptr / 4 + 1];
        if (rustptr === 0) return;
        const realRet = getStringFromWasm(rustptr, rustlen).slice();
        wasm.__wbindgen_free(rustptr, rustlen * 1);
        return realRet;

    };

    /**
    * @param {string} filter_id
    * @param {string} filter_body
    * @returns {string}
    */
    __exports.body_filter = function(filter_id, filter_body) {
        const ptr0 = passStringToWasm(filter_id);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm(filter_body);
        const len1 = WASM_VECTOR_LEN;
        const retptr = globalArgumentPtr();
        wasm.body_filter(retptr, ptr0, len0, ptr1, len1);
        const mem = getUint32Memory();
        const rustptr = mem[retptr / 4];
        const rustlen = mem[retptr / 4 + 1];
        if (rustptr === 0) return;
        const realRet = getStringFromWasm(rustptr, rustlen).slice();
        wasm.__wbindgen_free(rustptr, rustlen * 1);
        return realRet;

    };

    /**
    * @param {string} filter_id
    * @returns {string}
    */
    __exports.body_filter_end = function(filter_id) {
        const ptr0 = passStringToWasm(filter_id);
        const len0 = WASM_VECTOR_LEN;
        const retptr = globalArgumentPtr();
        wasm.body_filter_end(retptr, ptr0, len0);
        const mem = getUint32Memory();
        const rustptr = mem[retptr / 4];
        const rustlen = mem[retptr / 4 + 1];
        if (rustptr === 0) return;
        const realRet = getStringFromWasm(rustptr, rustlen).slice();
        wasm.__wbindgen_free(rustptr, rustlen * 1);
        return realRet;

    };

    const heap = new Array(32);

    heap.fill(undefined);

    heap.push(undefined, null, true, false);

function getObject(idx) { return heap[idx]; }

__exports.__widl_f_debug_1_ = function(arg0) {
    console.debug(getObject(arg0));
};

__exports.__widl_f_error_1_ = function(arg0) {
    console.error(getObject(arg0));
};

__exports.__widl_f_info_1_ = function(arg0) {
    console.info(getObject(arg0));
};

__exports.__widl_f_log_1_ = function(arg0) {
    console.log(getObject(arg0));
};

__exports.__widl_f_warn_1_ = function(arg0) {
    console.warn(getObject(arg0));
};

__exports.__wbg_error_4bb6c2a97407129a = function(arg0, arg1) {
    let varg0 = getStringFromWasm(arg0, arg1);

    varg0 = varg0.slice();
    wasm.__wbindgen_free(arg0, arg1 * 1);

    console.error(varg0);
};

let heap_next = heap.length;

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

__exports.__wbg_new_59cb74e423758ede = function() {
    return addHeapObject(new Error());
};

__exports.__wbg_stack_558ba5917b466edd = function(ret, arg0) {

    const retptr = passStringToWasm(getObject(arg0).stack);
    const retlen = WASM_VECTOR_LEN;
    const mem = getUint32Memory();
    mem[ret / 4] = retptr;
    mem[ret / 4 + 1] = retlen;

};

__exports.__wbg_new_3a746f2619705add = function(arg0, arg1) {
    let varg0 = getStringFromWasm(arg0, arg1);
    return addHeapObject(new Function(varg0));
};

__exports.__wbg_call_f54d3a6dadb199ca = function(arg0, arg1) {
    return addHeapObject(getObject(arg0).call(getObject(arg1)));
};

__exports.__wbg_self_ac379e780a0d8b94 = function(arg0) {
    return addHeapObject(getObject(arg0).self);
};

__exports.__wbg_crypto_1e4302b85d4f64a2 = function(arg0) {
    return addHeapObject(getObject(arg0).crypto);
};

__exports.__wbg_getRandomValues_1b4ba144162a5c9e = function(arg0) {
    return addHeapObject(getObject(arg0).getRandomValues);
};

function getArrayU8FromWasm(ptr, len) {
    return getUint8Memory().subarray(ptr / 1, ptr / 1 + len);
}

__exports.__wbg_getRandomValues_1ef11e888e5228e9 = function(arg0, arg1, arg2) {
    let varg1 = getArrayU8FromWasm(arg1, arg2);
    getObject(arg0).getRandomValues(varg1);
};

__exports.__wbg_require_6461b1e9a0d7c34a = function(arg0, arg1) {
    let varg0 = getStringFromWasm(arg0, arg1);
    return addHeapObject(require(varg0));
};

__exports.__wbg_randomFillSync_1b52c8482374c55b = function(arg0, arg1, arg2) {
    let varg1 = getArrayU8FromWasm(arg1, arg2);
    getObject(arg0).randomFillSync(varg1);
};

__exports.__wbindgen_string_new = function(p, l) { return addHeapObject(getStringFromWasm(p, l)); };

__exports.__wbindgen_is_undefined = function(i) { return getObject(i) === undefined ? 1 : 0; };

__exports.__wbindgen_jsval_eq = function(a, b) { return getObject(a) === getObject(b) ? 1 : 0; };

__exports.__wbindgen_throw = function(ptr, len) {
    throw new Error(getStringFromWasm(ptr, len));
};

function dropObject(idx) {
    if (idx < 36) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

__exports.__wbindgen_object_drop_ref = function(i) { dropObject(i); };

function init(module) {
    let result;
    const imports = { './redirectionio': __exports };
    if (module instanceof URL || typeof module === 'string' || module instanceof Request) {

        const response = fetch(module);
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            result = WebAssembly.instantiateStreaming(response, imports)
            .catch(e => {
                console.warn("`WebAssembly.instantiateStreaming` failed. Assuming this is because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);
                return response
                .then(r => r.arrayBuffer())
                .then(bytes => WebAssembly.instantiate(bytes, imports));
            });
        } else {
            result = response
            .then(r => r.arrayBuffer())
            .then(bytes => WebAssembly.instantiate(bytes, imports));
        }
    } else {

        result = WebAssembly.instantiate(module, imports)
        .then(result => {
            if (result instanceof WebAssembly.Instance) {
                return { instance: result, module };
            } else {
                return result;
            }
        });
    }
    return result.then(({instance, module}) => {
        wasm = instance.exports;
        init.__wbindgen_wasm_module = module;

        return wasm;
    });
}

self.wasm_bindgen = Object.assign(init, __exports);

})();