#!perl

################################################################################
=comment

Perl Weekly Challenge 068
=========================

Task #2
-------
*Reorder List*

=cut
################################################################################

#--------------------------------------#
# Copyright © 2020 PerlMonk Athanasius #
#--------------------------------------#

#===============================================================================
package SinglyLinkedList;
#===============================================================================

use strict;
use warnings;

#-------------------------------------------------------------------------------
sub new
#-------------------------------------------------------------------------------
{
    my ($class, @args) = @_;

    my  $self = { head => undef };
    my  $curr;

    for my $arg (@args)
    {
        my $node = { datum => $arg, next => undef };

        if (defined $curr)
        {
            $curr = $curr->{next} = $node;
        }
        else
        {
            $curr = $self->{head} = $node;
        }
    }

    return bless $self, $class;
}

#-------------------------------------------------------------------------------
sub get_head
#-------------------------------------------------------------------------------
{
    my ($self) = @_;

    return $self->{head};
}

#-------------------------------------------------------------------------------
sub remove_tail
#-------------------------------------------------------------------------------
{
    my ($self) = @_;
    my  $tail;

    if (my $curr = $self->{head})
    {
        if ($curr->{next})
        {
            $curr = $curr->{next} while $curr->{next} && $curr->{next}{next};
            $tail = $curr->{next};
            $curr->{next} = undef;
        }
        else
        {
            $tail = $curr;
            $self->{head} = undef;
        }
    }

    return $tail;
}

#-------------------------------------------------------------------------------
sub insert
#-------------------------------------------------------------------------------
{
    my ($self, $curr, $node_to_add) = @_;
    my  $old_next;

    if (defined $self->{head} && defined $curr && $curr)
    {
        $old_next     = $curr->{next};
        $curr->{next} = $node_to_add;
    }
    else
    {
        $old_next     = $self->{head};
        $self->{head} = $node_to_add;
    }

    $node_to_add->{next} = $old_next;
}

#-------------------------------------------------------------------------------
sub print
#-------------------------------------------------------------------------------
{
    my ($self, $title) = @_;

    print $title if defined $title;

    if (my $curr = $self->{head})
    {
        while ($curr)
        {
            print   $curr->{datum};
            $curr = $curr->{next};

            print ' -> ' if defined $curr;
        }
    }
    else
    {
        print '<empty>';
    }

    print "\n";
}

################################################################################
1;
################################################################################
