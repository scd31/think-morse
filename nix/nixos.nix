inputs:
{
  config,
  lib,
  pkgs,
  ...
}:
let
  pkg = inputs.self.packages.${pkgs.system}.default;
  cfg = config.services.think-morse;
in
{
  options = {
    services.think-morse = {
      enable = lib.mkEnableOption "think-morse - Blink your thinkpad LED as morse code";

      text = lib.mkOption {
        type = lib.types.string;
        description = "Text message to encode";
      };

      wpm = lib.mkOption {
        type = lib.types.float;
        default = 15.0;
        description = "Morse code speed, in wpm (words per minute)";
      };
    };
  };

  config = lib.mkIf cfg.enable {
    systemd.services.think-morse = {
      serviceConfig = {
        Type = "simple";
        Restart = "always";

        DynamicUser = true;
        PrivateTmp = true;
        SupplementaryGroups = [ "video" ];
      };

      requires = [ ];
      description = "think-morse - Blink your thinkpad LED as morse code";
      wantedBy = [ "multi-user.target" ];

      script = "${pkg}/bin/think-morse --wpm ${lib.escapeShellArg cfg.wpm} --repeat ${lib.escapeShellArg cfg.text}";
    };
  };
}
