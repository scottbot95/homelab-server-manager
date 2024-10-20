{ self, ... }:
{
  flake.modules.default = 
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
        openFirewall = mkOption {
          type = types.bool;
          default = false;
          description = "Whether to open \${cfg.port} on the local firewall";
        };
      };

      config = mkIf cfg.enable {
        systemd.services.homelab-server-manager = {
          description = "Homelab Server Manager";

          wantedBy = [ "multi-user.target" ];
          after = [ "network.target" ];

          restartIfChanged = true;

          serviceConfig = {
            DynamicUser = true;
            ExecStart = "${cfg.package}/bin/server --addr ${cfg.address} --port ${toString cfg.port}";
            Restart = "always";
          };
        };

        networking.firewall = mkIf cfg.openFirewall {
          allowedTCPPorts = [ cfg.port ];
          allowedUDPPorts = [ cfg.port ];
        };
      };
    };
}