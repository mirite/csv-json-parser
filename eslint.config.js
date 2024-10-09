import { general } from "@mirite/eslint-config-mirite";

export default [
  ...general,
  {
    ignores: ["pkg/**", "**/pkg/**"],
  },
];
