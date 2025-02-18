{ self, ... }:
{
  flake.nixosModules.homelab-server-manager =
    { config, lib, pkgs, ... }:
    let
      cfg = config.services.homelab-server-manager;
    in with lib; {
      options.services.homelab-server-manager = {
        enable = mkEnableOption "homelab-server-manager Game server";
        package = mkOption {
          type = types.package;
          default = self.packages.${pkgs.system}.default;
          defaultText = "self.packages.\${system}.default";
          description = "Package to use for homelab-server-manager service. Allows customizing version";
        };
        port = mkOption {
          type = types.port;
          default = 8080;
          description = "Port to bind server to";
        };
        address = mkOption {
          type = types.str;
          default = "0.0.0.0";
          description = "Address to bind server to";
        };
        configFile = mkOption {
          type = types.nullOr types.str;
          default = null;
          description = "Path to the config file. DO NOT USE /nix/store PATHS.";
        };
        envFile = mkOption {
          type = types.str;
          description = ''
            Path an environment file. DO NOT USE /nix/store PATHS.

            Must contain DISCORD_CLIENT_ID and DISCORD_CLIENT_SECRET
          '';
        };
        publicUrl = mkOption {
          type = types.str;
          default = "http://localhost:${toString cfg.port}";
          defaultText = "http://localhost:\${cfg.port}";
          description = ''
            Public URL. Mainly used for OAuth redirection.
          '';
        };
        secure = mkOption {
          type = types.bool;
          default = false;
          description = "Whether server is behind a TLS proxy";
        };
        openFirewall = mkOption {
          type = types.bool;
          default = false;
          description = "Whether to open \${cfg.port} on the local firewall";
        };
      };

      config = mkIf cfg.enable {
        systemd.services.homelab-server-manager =
        let
          args = [
            "--addr=${cfg.address}"
            "--port=${toString cfg.port}"
            ] ++ (lib.optional (cfg.configFile != null) "--config-file=%d/config.json")
            ++ (lib.optional cfg.secure "--secure");
          argString = builtins.concatStringsSep " \\\n" args;
        in {
          description = "Homelab Server Manager";

          wantedBy = [ "multi-user.target" ];
          after = [ "network.target" ];

          restartIfChanged = true;

          serviceConfig = {
            DynamicUser = true;
            StateDirectory = "homelab-server-manager";
            WorkingDirectory = "%S/homelab-server-manager";
            ExecStart = "${cfg.package}/bin/server ${argString}";
            Restart = "always";

            EnvironmentFile = cfg.envFile;

            LoadCredential = lib.optional (cfg.configFile != null) "config.json:${cfg.configFile}";
          };

          environment = {
            HOST_URL = cfg.publicUrl;
          };
        };

        networking.firewall = mkIf cfg.openFirewall {
          allowedTCPPorts = [ cfg.port ];
          allowedUDPPorts = [ cfg.port ];
        };
      };
    };
}