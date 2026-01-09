import {
    UrlEncode,
    UrlDecode,
    Base64Encode,
    Base64Decode,
    Base32HexDecode,
    Base32HexEncode,
    HexEncode,
    HexDecode,
    HtmlDecode,
    HtmlEncode,
    Md5Hash,
    Sha1Hash,
    Sha2Hash,
} from "../../../wasm-package/encore.js";

describe("UrlEncoding", () => {
    it("Url encode string", () => {
        let encoder = new UrlEncode({ non_ascii: true, charset: "e" });
        let utf8Encode = new TextEncoder();
        let actual = encoder.apply(utf8Encode.encode("irongate @éé🥖"));
        let expected = utf8Encode.encode("irongat%65 @%C3%A9%C3%A9%F0%9F%A5%96");

        expect(equal(actual, expected)).toBeTruthy();
    });

    it("Url decode string", () => {
        let encoder = new UrlDecode();
        let utf8Encode = new TextEncoder();
        let actual = encoder.apply(
            utf8Encode.encode("irongate @%C3%A9%C3%A9%F0%9F%A5%96")
        );
        let expected = utf8Encode.encode("irongate @éé🥖");

        expect(equal(actual, expected)).toBeTruthy();
    });
});

describe("Base64 encoding", () => {
    it("Base64 encode bytes", () => {
        let encoder = new Base64Encode();
        let utf8Encode = new TextEncoder();
        let actual = encoder.apply(utf8Encode.encode("irongate"));
        let expected = utf8Encode.encode("aXJvbmdhdGU=");

        expect(equal(actual, expected)).toBeTruthy();
    });

    it("Base64 decode bytes", () => {
        let encoder = new Base64Decode();
        let utf8Encode = new TextEncoder();
        let actual = encoder.apply(utf8Encode.encode("aXJvbmdhdGU="));
        let expected = utf8Encode.encode("irongate");

        expect(equal(actual, expected)).toBeTruthy();
    });
});

describe("Base32Hex encoding", () => {
    it("Base32Hex encode bytes", () => {
        let encoder = new Base32HexEncode();
        let utf8Encode = new TextEncoder();
        let actual = encoder.apply(utf8Encode.encode("irongate"));
        let expected = utf8Encode.encode("D5P6URJ7C5Q6A===");

        expect(equal(actual, expected)).toBeTruthy();
    });

    it("Base32Hex decode bytes", () => {
        let encoder = new Base32HexDecode();
        let utf8Encode = new TextEncoder();
        let actual = encoder.apply(utf8Encode.encode("D5P6URJ7C5Q6A==="));
        let expected = utf8Encode.encode("irongate");

        expect(equal(actual, expected)).toBeTruthy();
    });
});

describe("Hex encoding", () => {
    it("Hex encode bytes", () => {
        let encoder = new HexEncode({
            format: "Upper",
            prefix: "",
            delimiter: ""
        });
        let utf8Encode = new TextEncoder();
        let actual = encoder.apply(utf8Encode.encode("irongate"));
        let expected = utf8Encode.encode("69726F6E67617465");

        expect(equal(actual, expected)).toBeTruthy();
    });

    it("Hex decode bytes", () => {
        let encoder = new HexDecode({
            prefix: "0x",
            delimiter: ""
        });
        let utf8Encode = new TextEncoder();
        let actual = encoder.apply(utf8Encode.encode("0x690x720x6f0x6e0x670x610x740x65"));
        let expected = utf8Encode.encode("irongate");

        expect(equal(actual, expected)).toBeTruthy();
    });
});

describe("Html encoding", () => {
    it("Html encode bytes", () => {
        let encoder = new HtmlEncode();
        let utf8Encode = new TextEncoder();
        let actual = encoder.apply(
            utf8Encode.encode('\\&<script>alert(1)</script>a"')
        );
        let expected = utf8Encode.encode(
            "&#39;&amp;&lt;script&gt;alert(1)&lt;/script&gt;a&quot;"
        );

        expect(equal(actual, expected)).toBeTruthy();
    });

    it("Html decode bytes", () => {
        let decoder = new HtmlDecode();
        let utf8Encode = new TextEncoder();
        let actual = decoder.apply(
            utf8Encode.encode(
                "&#39;&amp;&lt;script&gt;alert(1)&lt;/script&gt;a&quot;"
            )
        );
        let expected = utf8Encode.encode('\\&<script>alert(1)</script>a"');

        expect(equal(actual, expected)).toBeTruthy();
    });
});

describe("Hash", () => {
    it("Hash bytes with md5", () => {
        let encoder = new Md5Hash();
        let utf8Encode = new TextEncoder();
        let actual = encoder.apply(utf8Encode.encode("irongate"));

        let hexEncoder = new HexDecode({});
        let expected = hexEncoder.apply(
            utf8Encode.encode("0d14b5c7855daa1c6f5bf7e4e1fbd762")
        );

        expect(equal(actual, expected)).toBeTruthy();
    });

    it("Hash bytes with sha1", () => {
        let encoder = new Sha1Hash();
        let utf8Encode = new TextEncoder();
        let actual = encoder.apply(utf8Encode.encode("irongate"));

        let hexEncoder = new HexDecode({});
        let expected = hexEncoder.apply(
            utf8Encode.encode("a3f8ebacb218657ded646a2403d9d517960e23f7")
        );

        expect(equal(actual, expected)).toBeTruthy();
    });

    it("Hash bytes with sha2", () => {
        let encoder = new Sha2Hash({ version: "Sha256" });
        let utf8Encode = new TextEncoder();
        let actual = encoder.apply(utf8Encode.encode("irongate"));

        let hexEncoder = new HexDecode({});
        let expected = hexEncoder.apply(
            utf8Encode.encode(
                "f85b3ccfdb7d51eb7cd37395e7dd423b05da972de615671b147c38a8a4a8642e"
            )
        );

        expect(equal(actual, expected)).toBeTruthy();
    });
});

const equal = (buf1: Uint8Array, buf2: Uint8Array) => {
    if (buf1.byteLength != buf2.byteLength) return false;
    for (var i = 0; i != buf1.byteLength; i++) {
        if (buf1[i] != buf2[i]) return false;
    }
    return true;
};

export { };