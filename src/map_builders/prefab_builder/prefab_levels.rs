#[allow(dead_code)]
#[derive(PartialEq, Copy, Clone)]
pub struct PrefabLevel {
    pub template: &'static str,
    pub width: usize,
    pub height: usize,
}

#[allow(dead_code)]
pub const POPULATED_MAP: PrefabLevel = PrefabLevel {
    template: LEVEL_MAP,
    width: 80,
    height: 43,
};

#[allow(dead_code)]
const LEVEL_MAP: &str = "
############################################################
###########   #################     ###########   #   ######
#         #                            #          #        #
#              g   #             # #####          #        #
#  @      #      ##              #                #  g     #
#         #     #             g  #            !   #        #
#         #    #                 #                #        #
## ###############################                #        #
## #      #           #                  o        #        #
## # #### #            ##              %         ###       #
## # ####                ##                       #        #
## # ########### ##        ##                     #        #
## # ########### ##          #                    #        #
## # ###########                   #####          #        #
## #       #####      ##         ##     ##                 #
## #    %      #    # ##        #         #       #      g #
## # o  ^  #####   #  ##       # #  # #  # #      #        #
## #       #   #  #   ##       #  ##   ##  #      #        #
##             # #    ##      #   ##   ##   #     #        #
##  ##     ######     ##   !  #  #  # #  #  #     #        #
##  ##     #   #      ##   ^  #             #    ######    #
##             #      ##      #          o  #    ######    #
##    ^    #####      ##      #     ###     ###########    #
##    !##  #   #      ##       #   #   #   # ##########    #
##     ##      #      ##       # ^#     #^ # ##########    #
##     ##  #####g               #         #  ##########    #
##         #   #   T             ##     ##   ########## !  #
## ##          #                   #####     ########## ^  #
## ##     g#####                             ##########    #
##         #####      o                      ##########    #
################  %               T          ##########    #
#              #  ^                          ##########    #
#         ##   #                      o    T ##########    #
#         ##   #                       ###       ######    #
#  o           # #############        #####  ^   ######    #
#      ##      ################       #####      ######    #
#     #  #     # ###############     #######     ######    #
#     #  #     #               ## ##########     ######    #
#    #    #    #                     #######  >  ######    #
#    #    #    #                 g   #######     ######    #
#   #      #   #                     #######   o ######    #
#   #       #  #                     #######     ######    #
#  #        #  #    o                #######     ######    #
#  #         # #                      #####      ######    #
# #          # #                      #####      ######    #
# #           ############################################ #
#    #                                                   # #
#    #        ###############          ############      # #
#   #        #              # #      # ######     ## #   # #
#   #       #               #  #    #  ######     #      # #
#   #       #               #   ## #   ######     #      # #
#  #       #                #    ##    # ###   !  ##  #  # #
#  #      #                 #   #  #   #          # ##   # #
#  #     #                  #  #    ## #          #      # #
# #      #                  # #       ##          #      # #
# #     #                   ##         #          #      # #
# #                        #############          #      # #
########################## ##                     ######## #
##########################                                 #
############################################################
";
