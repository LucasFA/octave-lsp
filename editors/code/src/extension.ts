import * as vscode from "vscode";
// import * as lc from "vscode-languageclient/node";
import Ctx from "./Ctx";

let ctx: Ctx;
export async function activate(context: vscode.ExtensionContext) {
    ctx = await Ctx.create(context);
}

export function deactivate() {
    return ctx.dispose();
}