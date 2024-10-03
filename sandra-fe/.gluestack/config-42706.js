"use strict";
var config = (() => {
  var __defProp = Object.defineProperty;
  var __getOwnPropDesc = Object.getOwnPropertyDescriptor;
  var __getOwnPropNames = Object.getOwnPropertyNames;
  var __hasOwnProp = Object.prototype.hasOwnProperty;
  var __require = /* @__PURE__ */ ((x) => typeof require !== "undefined" ? require : typeof Proxy !== "undefined" ? new Proxy(x, {
    get: (a, b) => (typeof require !== "undefined" ? require : a)[b]
  }) : x)(function(x) {
    if (typeof require !== "undefined")
      return require.apply(this, arguments);
    throw Error('Dynamic require of "' + x + '" is not supported');
  });
  var __export = (target, all) => {
    for (var name in all)
      __defProp(target, name, { get: all[name], enumerable: true });
  };
  var __copyProps = (to, from, except, desc) => {
    if (from && typeof from === "object" || typeof from === "function") {
      for (let key of __getOwnPropNames(from))
        if (!__hasOwnProp.call(to, key) && key !== except)
          __defProp(to, key, { get: () => from[key], enumerable: !(desc = __getOwnPropDesc(from, key)) || desc.enumerable });
    }
    return to;
  };
  var __toCommonJS = (mod) => __copyProps(__defProp({}, "__esModule", { value: true }), mod);

  // gluestack-style.config.ts
  var gluestack_style_config_exports = {};
  __export(gluestack_style_config_exports, {
    config: () => config
  });
  var import_react = __require("./mock-42706.js");
  var config = (0, import_react.createConfig)({
    aliases: {
      bg: "backgroundColor",
      bgColor: "backgroundColor",
      rounded: "borderRadius",
      h: "height",
      w: "width"
    },
    tokens: {
      colors: {
        primary0: "#ffffff",
        primary400: "#c084fc",
        primary500: "#a855f7",
        primary600: "#9333ea"
      },
      space: {
        4: 16,
        5: 20,
        6: 24
      },
      radii: {
        sm: 4,
        md: 6
      },
      letterSpacings: {
        md: 0
      },
      lineHeights: {
        sm: 20,
        md: 22
      },
      fontWeights: {
        normal: "400",
        medium: "500"
      },
      fontSizes: {
        sm: 14,
        md: 16
      },
      mediaQueries: {
        sm: "@media (min-width: 480px)",
        md: "@media (min-width: 768px)"
      }
    },
    globalStyle: {
      variants: {
        shadow: {
          softShadow: {
            shadowOffset: {
              width: 0,
              height: 0
            },
            shadowRadius: 10,
            shadowOpacity: 0.1,
            _android: {
              shadowColor: "$primary500",
              elevation: 5,
              shadowOpacity: 0.05
            }
          }
        }
      }
    }
  });
  return __toCommonJS(gluestack_style_config_exports);
})();
module.exports = config;
