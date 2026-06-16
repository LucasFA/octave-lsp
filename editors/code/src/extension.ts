import * as vscode from "vscode";

import Ctx from "./Ctx";

let ctx: Ctx | undefined;

export async function activate(context: vscode.ExtensionContext) {
    ctx = await Ctx.create(context);
}

export function deactivate() {
    if (ctx) {
        return ctx.dispose();
    }
}
