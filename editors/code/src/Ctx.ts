import * as vscode from "vscode";
import Config from "./Config";
import * as lang_client from "vscode-languageclient/node";
import * as path from "path";

export default class Ctx implements vscode.Disposable {  
    private constructor(
        private readonly extCtx: vscode.ExtensionContext,
        readonly config: Config,
        readonly client: lang_client.LanguageClient,
        readonly serverPath: string
    ) { }
    // this._extCtx.subscriptions.push(this);

    static async create(extCtx: vscode.ExtensionContext): Promise<Ctx> {      
        const serverBaseName = "octave-lsp-server";
        const serverName = serverBaseName + (process.platform === "win32" ? ".exe" : "");
        const serverPath = path.join(extCtx.extensionPath, "server", "target", "debug", serverName);

        const serverOptions: lang_client.ServerOptions = {
            run: {
                command: serverPath,
            },
            debug: {
                command: serverPath,
                args: ["--debug", "--inspect=6009"],
            },
        };

        const clientOptions: lang_client.LanguageClientOptions = {
            documentSelector: [{ scheme: "file", language: "octave" }],
            outputChannelName: "Octave Language Server",
            outputChannel: vscode.window.createOutputChannel("Octave Language Server"),
            revealOutputChannelOn: lang_client.RevealOutputChannelOn.Error
        };

        const client = new lang_client.LanguageClient("Octave Language Server", serverOptions, clientOptions);

        const ctx = new Ctx(extCtx, new Config(extCtx), client, serverPath);
        ctx.extCtx.subscriptions.push(ctx);
        await ctx.client.start();
        return ctx;
    }

    public push(d: vscode.Disposable): void {
        this.extCtx.subscriptions.push(d);
    }

    // registerCommand(name: string, factory: (ctx: Ctx) => Cmd) {
    //     const fullName = `octave.${name}`;
    //     const cmd = factory(this);
    //     const d = vscode.commands.registerCommand(fullName, cmd);
    //     this.push(d);
    // }

    public dispose() {
        return this.client.stop();
    }
}

export type Cmd = (...args: any[]) => unknown;
