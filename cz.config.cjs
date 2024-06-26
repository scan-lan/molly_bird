module.exports = {
  scopes: ["ui", "logic", "build"],
  allowEmptyScopes: true,
  allowCustomScopes: false,
  emptyScopesAlias: "empty",
  markBreakingChangeMode: false,
  breaklineNumber: 80,
  breaklineChar: "|",
  skipQuestions: [
    "footer",
    "scope",
    "breaking",
    "footerPrefix",
    "confirmCommit",
  ],
  allowEmptyIssuePrefix: true,
  confirmColorize: true,
  maxHeaderLength: 80,
  scopeOverrides: undefined,
  defaultBody: "",
  defaultIssues: "",
  defaultScope: "",
  defaultSubject: "",
};
